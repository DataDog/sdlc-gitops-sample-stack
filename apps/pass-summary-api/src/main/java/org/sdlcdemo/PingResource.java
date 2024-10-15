// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024 Datadog, Inc.

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
