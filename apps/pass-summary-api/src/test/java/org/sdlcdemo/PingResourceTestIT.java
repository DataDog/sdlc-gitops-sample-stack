// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024 Datadog, Inc.

package org.sdlcdemo;

import io.quarkus.test.junit.QuarkusIntegrationTest;

@QuarkusIntegrationTest
class PingResourceTestIT extends PingResourceTest {
    // Execute the same tests but in packaged mode.
}
