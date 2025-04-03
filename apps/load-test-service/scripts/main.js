import http from 'k6/http';
import { sleep, check } from 'k6';
import { SharedArray } from 'k6/data';
import { scenario } from 'k6/execution';
import { config, testData } from './config.js';

// Configuration for test scenarios
export const options = {
  scenarios: {
    // Pass API testing
    pass_api: {
      executor: 'constant-arrival-rate',
      rate: config.load.passApi.rps,  // Use configured RPS
      timeUnit: '1s',
      duration: config.durations.default,
      preAllocatedVUs: 5,
      maxVUs: config.load.passApi.maxVUs,
      exec: 'testPassApi',
      tags: { service: 'pass-api' }
    },
    // Pass Image API testing
    pass_image_api: {
      executor: 'constant-arrival-rate',
      rate: config.load.passImageApi.rps,  // Use configured RPS
      timeUnit: '1s',
      duration: config.durations.default,
      preAllocatedVUs: 5,
      maxVUs: config.load.passImageApi.maxVUs,
      exec: 'testPassImageApi',
      tags: { service: 'pass-image-api' },
      startTime: config.durations.default // Start after the pass_api scenario
    },
    // Pass Summary API testing
    pass_summary_api: {
      executor: 'constant-arrival-rate',
      rate: config.load.passSummaryApi.rps,  // Use configured RPS
      timeUnit: '1s',
      duration: config.durations.default,
      preAllocatedVUs: 5,
      maxVUs: config.load.passSummaryApi.maxVUs,
      exec: 'testPassSummaryApi',
      tags: { service: 'pass-summary-api' },
      startTime: config.durations.default + config.durations.default // Start after the pass_image_api scenario
    },
    // Combined workflow testing
    combined_workflow: {
      executor: 'ramping-vus',
      startVUs: 1,
      stages: [
        { duration: '15s', target: 5 },
        { duration: '30s', target: 5 },
        { duration: '15s', target: 0 }
      ],
      exec: 'testCombinedWorkflow',
      tags: { service: 'combined' },
      startTime: config.durations.default + config.durations.default + config.durations.default // Start after the pass_summary_api scenario
    }
  },
  thresholds: config.thresholds
};

// Sample coordinates for image requests
const coordinates = new SharedArray('coordinates', function() {
  return testData.coordinates;
});

// Test the Pass API service
export function testPassApi() {
  const baseUrl = config.services.passApi;
  
  // GET all passes
  const getAllRes = http.get(`${baseUrl}/passes`, {
    tags: { endpoint: 'get_all_passes' }
  });
  check(getAllRes, {
    'status is 200': (r) => r.status === 200,
    'response has passes': (r) => r.json().length > 0
  });

  // GET passes by elevation (an intentionally expensive query)
  const range = testData.elevationRanges[Math.floor(Math.random() * testData.elevationRanges.length)];
  const elevationRes = http.get(`${baseUrl}/passes/elevation?min=${range.min}&max=${range.max}`, {
    tags: { endpoint: 'get_passes_by_elevation' }
  });
  check(elevationRes, {
    'status is 200': (r) => r.status === 200
  });

  sleep(1);
}

// Test the Pass Image API service
export function testPassImageApi() {
  const baseUrl = config.services.passImageApi;
  const passApiUrl = config.services.passApi;
  
  // Get pass data first (to simulate a real flow)
  const passesRes = http.get(`${passApiUrl}/passes`, {
    tags: { endpoint: 'get_all_passes' }
  });
  
  if (passesRes.status === 200) {
    const passes = passesRes.json();
    
    if (passes.length > 0) {
      // Pick a random pass that has coordinates
      const validPasses = passes.filter(p => p.latitude && p.longitude);
      
      if (validPasses.length > 0) {
        const pass = validPasses[Math.floor(Math.random() * validPasses.length)];
        
        // Request image through the pass-api /passes/{id}/image endpoint
        const passImgRes = http.get(`${passApiUrl}/passes/${pass.id}/image`, {
          tags: { endpoint: 'get_pass_image_via_api' }
        });
        
        check(passImgRes, {
          'pass-api image status is 200': (r) => r.status === 200,
          'pass-api image content-type is image': (r) => r.headers['Content-Type'] && r.headers['Content-Type'].includes('image/')
        });
      }
    }
  }
  
  // Also test the pass-image-api service directly with some fixed coordinates
  const coord = coordinates[Math.floor(Math.random() * coordinates.length)];
  const imgRes = http.get(`${baseUrl}/images/${coord.long}/${coord.lat}/${coord.size}?radius=${coord.radius}`, {
    tags: { endpoint: 'get_direct_image' }
  });
  
  check(imgRes, {
    'direct image status is 200': (r) => r.status === 200,
    'direct image content-type is image': (r) => r.headers['Content-Type'] && r.headers['Content-Type'].includes('image/')
  });
  
  sleep(1);
}

// Test the Pass Summary API service
export function testPassSummaryApi() {
  const baseUrl = config.services.passSummaryApi;
  
  // GET summary
  const summaryRes = http.get(`${baseUrl}/pass-summary`, {
    tags: { endpoint: 'get_pass_summary' }
  });
  check(summaryRes, {
    'status is 200': (r) => r.status === 200,
    'has pass_count': (r) => r.json().pass_count !== undefined,
    'has total_ascent': (r) => r.json().total_ascent !== undefined
  });
  
  // Occasionally test the slow endpoint to simulate problematic traffic
  if (Math.random() < 0.2) { // 20% of requests go to the slow endpoint
    const slowRes = http.get(`${baseUrl}/pass-summary/slow`, {
      tags: { endpoint: 'get_pass_summary_slow' }
    });
    check(slowRes, {
      'slow endpoint status is 200': (r) => r.status === 200
    });
  }
  
  sleep(1);
}

// Test a combined workflow that touches all services
export function testCombinedWorkflow() {
  const passApiUrl = config.services.passApi;
  const passImageApiUrl = config.services.passImageApi;
  const passSummaryApiUrl = config.services.passSummaryApi;
  
  // 1. First get the summary data
  const summaryRes = http.get(`${passSummaryApiUrl}/pass-summary`, {
    tags: { endpoint: 'workflow_summary' }
  });
  
  // 2. Then fetch all passes
  const passesRes = http.get(`${passApiUrl}/passes`, {
    tags: { endpoint: 'workflow_passes' }
  });
  
  if (passesRes.status === 200) {
    const passes = passesRes.json();
    
    if (passes.length > 0) {
      // 3. Pick a pass and get its details
      const passId = passes[Math.floor(Math.random() * passes.length)].id;
      
      const passDetailRes = http.get(`${passApiUrl}/passes/${passId}`, {
        tags: { endpoint: 'workflow_pass_detail' }
      });
      
      // 4. Get the image for this pass if it has coordinates
      const passDetail = passDetailRes.json();
      if (passDetail && passDetail.latitude && passDetail.longitude) {
        const passImgRes = http.get(`${passApiUrl}/passes/${passId}/image`, {
          tags: { endpoint: 'workflow_pass_image' }
        });
      }
    }
  }
  
  sleep(2);
} 