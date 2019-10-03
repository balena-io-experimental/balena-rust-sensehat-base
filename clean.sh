#!/bin/bash

PROJECT_NAME_UNDERSCORED="$(sed s/[-]/_/g <<<$PROJECT_NAME)"
rm -rf target/release/$PROJECT_NAME* target/release/deps/$PROJECT_NAME_UNDERSCORED-*
