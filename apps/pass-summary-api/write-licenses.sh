#!/bin/bash

# Define the input XML file
INPUT_FILE="target/generated-resources/licenses.xml"
OUTPUT_FILE="LICENSE-3rdparty.csv"

# Write the header to the CSV file
echo "Component,Origin,License,Copyright" > "$OUTPUT_FILE"

# Debugging: Confirm input and output file paths
echo "Input file: $INPUT_FILE"
echo "Output file: $OUTPUT_FILE"

# Parse the XML file and extract the necessary data using xmllint and awk
xmllint --format "$INPUT_FILE" 2>/dev/null | awk '
/<dependency>/ { insideDependency = 1 }
/<\/dependency>/ {
    block = block $0 "\n";  # Include the </dependency> line
    insideDependency = 0;
    print block;
    block = "";
    next
}

{
    if (insideDependency) {
        block = block $0 "\n"
    }
}
' | awk '
/<groupId>/ {
    gsub(/.*<groupId>|<\/groupId>.*/, "", $0);
    groupId=$0;
    print "groupId: " groupId > "/dev/stderr";
}

/<artifactId>/ {
    gsub(/.*<artifactId>|<\/artifactId>.*/, "", $0);
    artifactId=$0;
    print "artifactId: " artifactId > "/dev/stderr";
}

/<version>/ {
    gsub(/.*<version>|<\/version>.*/, "", $0);
    version=$0;
    print "version: " version > "/dev/stderr";
}

/<name>/ {
    gsub(/.*<name>|<\/name>.*/, "", $0);
    licenseName=$0;
    print "licenseName: " licenseName > "/dev/stderr";
}

/<url>/ {
    gsub(/.*<url>|<\/url>.*/, "", $0);
    url=$0;
    print "url: " url > "/dev/stderr";
}

/<\/dependency>/ {
    # Construct the Origin field
    origin = groupId "/" artifactId ":" version ;
    # Debugging: Print what will be written to the output file
    print "WRITING TO CSV: " artifactId "," origin "," licenseName ",Copyright Placeholder" > "/dev/stderr";
    # Output in the desired format
    printf "pass-summary-api,%s,%s,Copyright Placeholder\n", origin, licenseName >> "'"$OUTPUT_FILE"'"
}'

echo "CSV generated at $OUTPUT_FILE"
