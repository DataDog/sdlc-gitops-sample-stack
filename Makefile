license-raw-pass-api.csv: apps/pass-api/go.sum
	cd apps/pass-api && \
	./write-licenses.sh && \
	cp LICENSE-3rdparty.csv ../../license-raw-pass-api.csv

# This one is built by hand from the direct deps only
license-raw-summary-api.csv: apps/pass-summary-api/LICENSE-3rdparty.csv
	cp apps/pass-summary-api/LICENSE-3rdparty.csv license-raw-summary-api.csv

license-raw-combined.csv: license-raw-pass-api.csv license-raw-summary-api.csv
	cat license-raw-pass-api.csv > license-raw-combined.csv
	tail -n +2 license-raw-summary-api.csv >> license-raw-combined.csv

#
# Clean up all the licenses that get reported out of our dependencies 
# so that they use clean SPDX identifiers. 
#
# Bad bad not good but does what it says on the tin
# 
LICENSE-3rdparty.csv: license-raw-combined.csv
	sed -e 's|Apache License 2.0|Apache-2.0|g' \
	    -e 's|Apache License, Version 2.0|Apache-2.0|g' \
	    -e 's|The Apache Software License, Version 2.0|Apache-2.0|g' \
	    -e 's|Apache License Version 2.0|Apache-2.0|g' \
	    -e 's|Apache 2.0|Apache-2.0|g' \
	    -e 's|The Apache Software License|Apache-2.0|g' \
		-e 's|Apache Software License - Version 2.0|Apache-2.0|g' \
		-e 's|The Apache-2.0|Apache-2.0|g' \
	    -e 's|The MIT License|MIT|g' \
	    -e 's|MIT License|MIT|g' \
	    -e 's|BSD License 3|BSD-3-Clause|g' \
	    -e 's|The BSD 3-Clause License|BSD-3-Clause|g' \
	    -e 's|Eclipse Public License - v 1.0|EPL-1.0|g' \
		-e 's|EPL-2.0 - Version 1.0|EPL-2.0|g' \
		-e 's|EPL-2.0 v2.0|EPL-2.0|g' \
	    -e 's|Eclipse Public License - v 2.0|EPL-2.0|g' \
	    -e 's|Eclipse Public License, Version 2.0|EPL-2.0|g' \
	    -e 's|Eclipse Public License|EPL-2.0|g' \
		-e 's|EPL-2.0 - Version 1.0|EPL-2.0|g' \
	    -e 's|EDL 1.0|EDL-1.0|g' \
	    -e 's|Eclipse Distribution License - v 1.0|EDL-1.0|g' \
	    -e 's|Eclipse Distribution License|EDL-1.0|g' \
	    -e 's|GPL2 w/ CPE|GPL-2.0-with-classpath-exception|g' \
	    -e 's|GNU General Public License, version 2 with the GNU Classpath Exception|GPL-2.0-with-classpath-exception|g' \
		-e 's|CDDL-1.1 AND GPL-2.0-only WITH Classpath-exception-2.0|GPL-2.0-with-classpath-exception|g' \
	    -e 's|CDDL + GPLv2 with classpath exception|GPL-2.0-with-classpath-exception|g' \
	    -e 's|Universal Permissive License, Version 1.0|UPL-1.0|g' \
	    -e 's|MIT-0|MIT-0|g' \
	    license-raw-combined.csv > LICENSE-3rdparty.csv

#
# License overrides:
# Taken from NOTICE --> https://github.com/hyperxpro/Brotli4j/blob/main/NOTICE.txt
# com.aayushatharva.brotli4j*,Copyright 2021, Aayush Atharva

# Taken from source --> https://github.com/search?q=repo%3AFasterXML%2Fjackson-core%20copyright&type=code
# com.fasterxml.jackson.core*,Copyright (c) 2007- Tatu Saloranta, tatu.saloranta@iki.fi, 

# Take from NOTICE --> https://github.com/netty/netty/blob/4.1/NOTICE.txt
# io.netty/*, Copyright 2014 The Netty Project

# Everything from Quarkus
# io.quarkus*, Copyright The Quarkus Authors

# Everything from Smallrye
# io.smallrye*, Copyright The SmallRye Authors

# Everything from Eclipse vert.x -https://github.com/eclipse-vertx/vert.x/blob/6d98edbd7c87d4042c8f17e02b2bb889dee4d764/NOTICE.md?plain=1#L23
# io.vertx*, Copyright the Eclipse Vert.x Authors

# Everything from rest-assured - https://github.com/search?q=repo%3Arest-assured%2Frest-assured%20copyright&type=code
# io.rest-assured, Copyright 2019 the original author or authors.

# Everything Jakarta
# jakarta.*, Copyright The Jakarta EE Contributors

# Everything Apache
# org.apache*, Copyright The Apache Software Foundation


all: LICENSE-3rdparty.csv

clean: 
	rm license-raw-* LICENSE-3rdparty.csv