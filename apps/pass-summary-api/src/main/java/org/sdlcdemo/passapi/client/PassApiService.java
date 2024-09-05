package org.sdlcdemo.passapi.client;

import jakarta.ws.rs.*;
import org.eclipse.microprofile.rest.client.inject.RegisterRestClient;

import java.util.Set;

@Path("/passes")
@RegisterRestClient(configKey = "pass-api-client")
public interface PassApiService {

    @GET
    Set<Pass> all();

    @GET
    @Path("/{id}")
    Set<Pass> passId(@PathParam("id") String id);

}
