// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024 Datadog, Inc.

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
    Pass getById(@PathParam("id") int id);

}
