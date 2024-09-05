package org.sdlcdemo;

import jakarta.ws.rs.*;
import jakarta.ws.rs.core.MediaType;
import java.util.Collections;
import java.util.Map;

@Path("/ping")
public class PingResource {

    @GET
    @Produces(MediaType.APPLICATION_JSON)
    public Map<String, Boolean> ping() {
        return Collections.singletonMap("ok", true);
    }
}
