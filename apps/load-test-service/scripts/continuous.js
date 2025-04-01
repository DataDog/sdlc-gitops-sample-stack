import http from 'k6/http';
import { sleep, check } from 'k6';
import { SharedArray } from 'k6/data';
import { config, testData } from './config.js';

// Configuration for continuous testing
export const options = {
  scenarios: {
    // Continuous testing of all services with lower load for extended periods
    continuous: {
      executor: 'constant-arrival-rate',
      rate: Math.max(1, Math.floor(config.load.passApi.rps / 5)),  // Lower RPS for continuous testing
      timeUnit: '1s',
      duration: config.durations.continuous,
      preAllocatedVUs: 2,
      maxVUs: 10,
      exec: 'continuousWorkflow'
    }
  },
  thresholds: config.thresholds
};

// Sample coordinates for image requests
const coordinates = new SharedArray('coordinates', function() {
  return testData.coordinates;
});

// Continuous workflow that periodically tests all services
export function continuousWorkflow() {
  const passApiUrl = config.services.passApi;
  const passImageApiUrl = config.services.passImageApi;
  const passSummaryApiUrl = config.services.passSummaryApi;
  
  // Randomly select which service to test
  const serviceSelector = Math.random();
  
  if (serviceSelector < 0.4) {
    // 40% of requests go to pass-api
    testPassApi(passApiUrl);
  } else if (serviceSelector < 0.7) {
    // 30% of requests go to pass-summary-api
    testPassSummaryApi(passSummaryApiUrl);
  } else if (serviceSelector < 0.9) {
    // 20% of requests go to pass-image-api
    testPassImageApi(passApiUrl, passImageApiUrl);
  } else {
    // 10% of requests run the combined workflow
    runCombinedWorkflow(passApiUrl, passImageApiUrl, passSummaryApiUrl);
  }
  
  // Varied sleep between 1-3 seconds
  sleep(1 + Math.random() * 2);
}

function testPassApi(baseUrl) {
  // GET all passes or elevation query based on probability
  if (Math.random() < 0.7) {
    // 70% of the time, get all passes
    const getAllRes = http.get(`${baseUrl}/passes`, {
      tags: { endpoint: 'get_all_passes', mode: 'continuous' }
    });
    
    check(getAllRes, {
      'status is 200': (r) => r.status === 200
    });
  } else {
    // 30% of the time, do an elevation query
    const range = testData.elevationRanges[Math.floor(Math.random() * testData.elevationRanges.length)];
    
    const elevationRes = http.get(`${baseUrl}/passes/elevation?min=${range.min}&max=${range.max}`, {
      tags: { endpoint: 'get_passes_by_elevation', mode: 'continuous' }
    });
    
    check(elevationRes, {
      'status is 200': (r) => r.status === 200
    });
  }
}

function testPassImageApi(passApiUrl, imageApiUrl) {
  // Either use direct coordinates or go through the pass-api
  if (Math.random() < 0.5) {
    // 50% direct to image API
    const coord = coordinates[Math.floor(Math.random() * coordinates.length)];
    
    const imgRes = http.get(`${imageApiUrl}/images/${coord.long}/${coord.lat}/${coord.size}?radius=${coord.radius}`, {
      tags: { endpoint: 'get_direct_image', mode: 'continuous' }
    });
    
    check(imgRes, {
      'direct image status is 200': (r) => r.status === 200
    });
  } else {
    // 50% through pass-api
    const passesRes = http.get(`${passApiUrl}/passes`, {
      tags: { endpoint: 'get_all_passes', mode: 'continuous' }
    });
    
    if (passesRes.status === 200) {
      const passes = passesRes.json();
      
      if (passes.length > 0) {
        // Find passes with coordinates
        const validPasses = passes.filter(p => p.latitude && p.longitude);
        
        if (validPasses.length > 0) {
          const pass = validPasses[Math.floor(Math.random() * validPasses.length)];
          
          const passImgRes = http.get(`${passApiUrl}/passes/${pass.id}/image`, {
            tags: { endpoint: 'get_pass_image_via_api', mode: 'continuous' }
          });
          
          check(passImgRes, {
            'pass-api image status is 200': (r) => r.status === 200
          });
        }
      }
    }
  }
}

function testPassSummaryApi(baseUrl) {
  // Either use normal or slow endpoint
  if (Math.random() < 0.9) {
    // 90% of the time, use normal endpoint
    const summaryRes = http.get(`${baseUrl}/pass-summary`, {
      tags: { endpoint: 'get_pass_summary', mode: 'continuous' }
    });
    
    check(summaryRes, {
      'status is 200': (r) => r.status === 200
    });
  } else {
    // 10% of the time, use slow endpoint
    const slowRes = http.get(`${baseUrl}/pass-summary/slow`, {
      tags: { endpoint: 'get_pass_summary_slow', mode: 'continuous' }
    });
    
    check(slowRes, {
      'slow endpoint status is 200': (r) => r.status === 200
    });
  }
}

function runCombinedWorkflow(passApiUrl, passImageApiUrl, passSummaryApiUrl) {
  // First get summary data
  http.get(`${passSummaryApiUrl}/pass-summary`, {
    tags: { endpoint: 'workflow_summary', mode: 'continuous' }
  });
  
  // Then get all passes
  const passesRes = http.get(`${passApiUrl}/passes`, {
    tags: { endpoint: 'workflow_passes', mode: 'continuous' }
  });
  
  if (passesRes.status === 200) {
    const passes = passesRes.json();
    
    if (passes.length > 0) {
      // Get a random pass detail
      const passId = passes[Math.floor(Math.random() * passes.length)].id;
      
      const passDetailRes = http.get(`${passApiUrl}/passes/${passId}`, {
        tags: { endpoint: 'workflow_pass_detail', mode: 'continuous' }
      });
      
      // If it has coordinates, get the image
      const passDetail = passDetailRes.json();
      if (passDetail && passDetail.latitude && passDetail.longitude) {
        http.get(`${passApiUrl}/passes/${passId}/image`, {
          tags: { endpoint: 'workflow_pass_image', mode: 'continuous' }
        });
      }
    }
  }
} 