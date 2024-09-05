package org.sdlcdemo;

import io.quarkus.test.common.QuarkusTestResource;
import io.quarkus.test.junit.QuarkusTest;
import org.junit.jupiter.api.Test;

import static io.restassured.RestAssured.given;
import static org.hamcrest.CoreMatchers.is;

@QuarkusTest
@QuarkusTestResource(WireMockExtensions.class)
class DownstreamResourceTest {
    @Test
    void testHelloEndpoint() {
        given()
          .when().get("/downstream")
          .then()
             .statusCode(200)
             .body(is("{\"total_ascent\":6518,\"pass_count\":3}"));
    }

}