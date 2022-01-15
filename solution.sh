#!/bin/sh
#TODO options transit, angle
cargo run --example svg_transit -- --natal_lat 46.12 --transit_lat 46.12 --natal_lng 6.09 --transit_lng 6.09 --natal_time_zone +2 --transit_time_zone +2 --natal_date 03.04.1986 --transit_date 12.1.2022 --natal_time 04:54 --transit_time 12:00 --path_export ~/stephane.svg --path_ephem /Users/stephane/ephe/ephem_files -a 0
cargo run --example svg -- 46.12 6.09 +2 -d 03.04.1986 -t 4:56 --path_export ~/theme.svg --path_ephem /Users/stephane/ephe/ephem_files