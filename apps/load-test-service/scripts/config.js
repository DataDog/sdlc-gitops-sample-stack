// Configuration for load test service
export const config = {
  // Default service URLs
  services: {
    passApi: __ENV.PASS_API_URL || 'http://pass-api:8080',
    passImageApi: __ENV.PASS_IMAGE_API_URL || 'http://pass-image-api:8080',
    passSummaryApi: __ENV.PASS_SUMMARY_API_URL || 'http://pass-summary-api:8080'
  },
  
  // Test durations
  durations: {
    default: '30s',
    combined: '60s',
    continuous: '24h'
  },
  
  // Load settings
  load: {
    passApi: {
      rps: parseInt(__ENV.PASS_API_RPS || '2'),
      maxVUs: parseInt(__ENV.PASS_API_MAX_VUS || '20')
    },
    passImageApi: {
      rps: parseInt(__ENV.PASS_IMAGE_API_RPS || '1'),
      maxVUs: parseInt(__ENV.PASS_IMAGE_API_MAX_VUS || '10')
    },
    passSummaryApi: {
      rps: parseInt(__ENV.PASS_SUMMARY_API_RPS || '2'),
      maxVUs: parseInt(__ENV.PASS_SUMMARY_API_MAX_VUS || '20')
    }
  },
  
  // Thresholds
  thresholds: {
    http_req_duration: ['p(95)<500'], // 95% of requests should be below 500ms
  }
};

// Sample test data
export const testData = {
  // Sample mountain passes with coordinates
  coordinates: [
    { name: 'Grosse Scheidegg', lat: 46.655559, long: 8.102121, radius: 1.5, size: 500 },
    { name: 'Stelvio Pass', lat: 46.5309, long: 10.4515, radius: 1.2, size: 500 },
    { name: 'Alpe d\'Huez', lat: 45.0909, long: 6.0736, radius: 1.0, size: 500 },
    { name: 'Col du Galibier', lat: 45.0644, long: 6.4077, radius: 1.3, size: 500 },
    { name: 'Passo Giau', lat: 46.4833, long: 12.0667, radius: 1.1, size: 500 }
  ],
  
  // Elevation ranges for searches
  elevationRanges: [
    { min: 1000, max: 2000 },
    { min: 1500, max: 2500 },
    { min: 2000, max: 3000 },
    { min: 1000, max: 3000 }
  ]
}; 