#!/bin/bash --
# Copyright 2018 POA Networks, Ltd.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# and limitations under the License.

set -euo pipefail

case $# in
   (2|3) :;;
   (*) echo 'Must have 2 or 3 arguments' >&2; exit 1;;
esac

cmp <(~/.local/bin/pot -tpietrzak "-l${3-2048}" -- "$1" "$2") \
    <(./vdf-cli prove -- "$@")
