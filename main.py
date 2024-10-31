import sys
import argparse
from myLib.extract import extract
from myLib.transform_load import load
from myLib.query import (
    update_record,
    delete_record,
    create_record,
    general_query,
    read_data,
)


def handle_arguments(args):
    """Add action based on initial calls"""
    parser = argparse.ArgumentParser(description="ETL-Query script for AirlineSafetyDB")
    parser.add_argument(
        "action",
        choices=[
            "extract",
            "transform_load",
            "update_record",
            "delete_record",
            "create_record",
            "general_query",
            "read_data",
        ],
    )
    args = parser.parse_args(args[:1])
    print(args.action)

    if args.action == "update_record":
        parser.add_argument("record_id", type=int)
        parser.add_argument("airline")
        parser.add_argument("avail_seat_km_per_week", type=int)
        parser.add_argument("incidents_85_99", type=int)
        parser.add_argument("fatal_accidents_85_99", type=int)
        parser.add_argument("fatalities_85_99", type=int)
        parser.add_argument("incidents_00_14", type=int)
        parser.add_argument("fatal_accidents_00_14", type=int)
        parser.add_argument("fatalities_00_14", type=int)

    if args.action == "create_record":
        parser.add_argument("airline")
        parser.add_argument("avail_seat_km_per_week", type=int)
        parser.add_argument("incidents_85_99", type=int)
        parser.add_argument("fatal_accidents_85_99", type=int)
        parser.add_argument("fatalities_85_99", type=int)
        parser.add_argument("incidents_00_14", type=int)
        parser.add_argument("fatal_accidents_00_14", type=int)
        parser.add_argument("fatalities_00_14", type=int)

    if args.action == "general_query":
        parser.add_argument("query")

    if args.action == "delete_record":
        parser.add_argument("record_id", type=int)

    # Parse again with every argument
    return parser.parse_args(sys.argv[1:])


def main():
    """Handles all the CLI commands"""
    args = handle_arguments(sys.argv[1:])

    if args.action == "extract":
        print("Extracting data...")
        extract()
    elif args.action == "transform_load":
        print("Transforming data...")
        load()
    elif args.action == "update_record":
        update_record(
            args.record_id,
            args.airline,
            args.avail_seat_km_per_week,
            args.incidents_85_99,
            args.fatal_accidents_85_99,
            args.fatalities_85_99,
            args.incidents_00_14,
            args.fatal_accidents_00_14,
            args.fatalities_00_14,
        )
    elif args.action == "delete_record":
        delete_record(args.record_id)
    elif args.action == "create_record":
        create_record(
            args.airline,
            args.avail_seat_km_per_week,
            args.incidents_85_99,
            args.fatal_accidents_85_99,
            args.fatalities_85_99,
            args.incidents_00_14,
            args.fatal_accidents_00_14,
            args.fatalities_00_14,
        )
    elif args.action == "general_query":
        general_query(args.query)
    elif args.action == "read_data":
        data = read_data()
        print(data)
    else:
        print(f"Unknown action: {args.action}")


if __name__ == "__main__":
    main()