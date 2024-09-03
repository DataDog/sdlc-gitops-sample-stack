package org.sdlcdemo;

import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;
import org.eclipse.microprofile.rest.client.inject.RestClient;
import org.sdlcdemo.simpleapi.client.Pass;
import org.sdlcdemo.simpleapi.client.SimpleApiService;

import java.util.HashMap;
import java.util.Map;
import java.util.Set;

/**
 * A resource that is downstream of simple-api
 */
@Path("/downstream")
public class DownstreamResource {

    @RestClient
    SimpleApiService simpleApiService;

    @GET
    @Produces(MediaType.APPLICATION_JSON)
    public Map<String, Object> passSummary() {
        Set<Pass> allPasses = simpleApiService.all();

        int totalAscent = allPasses.stream()
                .mapToInt(Pass::ascent)
                .sum();

        Map<String, Object> response = new HashMap<>();
        response.put("pass_count", allPasses.size());
        response.put("total_ascent", totalAscent);

        return response;
    }
}
