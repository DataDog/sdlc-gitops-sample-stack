package org.sdlcdemo;

import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;
import org.eclipse.microprofile.rest.client.inject.RestClient;
import org.sdlcdemo.passapi.client.Pass;
import org.sdlcdemo.passapi.client.PassApiService;

import java.util.HashMap;
import java.util.Map;
import java.util.Set;

/**
 * A resource that provides pass summary information 
 * using pass information received from the pass-api
 */
@Path("/pass-summary")
public class PassSummaryResource {

    @RestClient
    PassApiService passApiService;

    @GET
    @Produces(MediaType.APPLICATION_JSON)
    public Map<String, Object> passSummary() {
        Set<Pass> allPasses = passApiService.all();

        int totalAscent = allPasses.stream()
                .mapToInt(Pass::ascent)
                .sum();

        Map<String, Object> response = new HashMap<>();
        response.put("pass_count", allPasses.size());
        response.put("total_ascent", totalAscent);

        return response;
    }
}
