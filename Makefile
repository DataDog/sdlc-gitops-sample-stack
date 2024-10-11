license-raw-pass-api.csv: apps/pass-api/go.sum
	cd apps/pass-api && \
	./write-licenses.sh && \
	cp LICENSE-3rdparty.csv ../../license-raw-pass-api.csv

license-raw-summary-api.csv: apps/pass-summary-api/pom.xml
	cd apps/pass-summary-api && \
	./write-licenses.sh && \
	cp LICENSE-3rdparty.csv ../../license-raw-summary-api.csv

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

all: LICENSE-3rdparty.csv

clean: 
	rm license-raw-* LICENSE-3rdparty.csv