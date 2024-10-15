// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024 Datadog, Inc.

package org.sdlcdemo;

import io.quarkus.test.common.QuarkusTestResource;
import io.quarkus.test.junit.QuarkusTest;
import org.junit.jupiter.api.Test;

import static io.restassured.RestAssured.given;
import static org.hamcrest.CoreMatchers.is;

@QuarkusTest
@QuarkusTestResource(WireMockExtensions.class)
class PassSummaryResourceTest {
    @Test
    void testHelloEndpoint() {
        given()
          .when().get("/pass-summary")
          .then()
             .statusCode(200)
             .body(is("{\"total_ascent\":6518,\"pass_count\":3}"));
    }

}
