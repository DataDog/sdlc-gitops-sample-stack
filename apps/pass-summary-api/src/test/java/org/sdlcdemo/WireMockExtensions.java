// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024 Datadog, Inc.

package org.sdlcdemo;

import java.util.Map;

import com.github.tomakehurst.wiremock.WireMockServer;
import io.quarkus.test.common.QuarkusTestResourceLifecycleManager;

import static com.github.tomakehurst.wiremock.client.WireMock.*;

public class WireMockExtensions implements QuarkusTestResourceLifecycleManager {

    private WireMockServer wireMockServer;

    @Override
    public Map<String, String> start() {
        wireMockServer = new WireMockServer();
        wireMockServer.start();

        wireMockServer.stubFor(get(urlEqualTo("/passes"))
                .willReturn(aResponse()
                        .withHeader("Content-Type", "application/json")
                        .withBody("""
                        [
                            {
                                "id": 1,
                                "name": "Grosse Scheidegg",
                                "country": "Switzerland",
                                "ascent": 1900
                            },
                            {
                                "id": 2,
                                "name": "Stelvio Pass",
                                "country": "Italy",
                                "ascent": 2758
                            },
                            {
                                "id": 3,
                                "name": "Alpe d'Huez",
                                "country": "France",
                                "ascent": 1860
                            }
                        ]
                        """)));

        return Map.of("quarkus.rest-client.pass-api-client.url", wireMockServer.baseUrl());
    }

    @Override
    public void stop() {
        if (null != wireMockServer) {
            wireMockServer.stop();
        }
    }
}
