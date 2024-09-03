package org.sdlcdemo.simpleapi.client;

import jakarta.ws.rs.*;
import jakarta.ws.rs.core.MediaType;
import org.eclipse.microprofile.rest.client.inject.RegisterRestClient;

import java.util.Set;

@Path("/passes")
@RegisterRestClient(configKey = "simple-api-client")
public interface SimpleApiService {

    @GET
    Set<Pass> all();

    @GET
    @Path("/{id}")
    Set<Pass> passId(@PathParam("id") String id);

}
