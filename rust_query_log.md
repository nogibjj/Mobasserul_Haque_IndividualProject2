```sql
SELECT * FROM AirlineSafety LIMIT 10;
```

```sql
INSERT INTO AirlineSafety (airline, avail_seat_km_per_week, incidents_85_99, fatal_accidents_85_99, fatalities_85_99, incidents_00_14, fatal_accidents_00_14, fatalities_00_14) VALUES ('Air Canada', 1865253802, 2, 0, 0, 2, 0, 0);
```

```sql
UPDATE AirlineSafety SET airline = 'Air Canada', avail_seat_km_per_week = 2000000000, incidents_85_99 = 3, fatal_accidents_85_99 = 1, fatalities_85_99 = 5, incidents_00_14 = 2, fatal_accidents_00_14 = 0, fatalities_00_14 = 0 WHERE id = 1;
```

```sql
DELETE FROM AirlineSafety WHERE id = 1;
```

```sql
SELECT * FROM AirlineSafety LIMIT 10;
```

```sql
INSERT INTO AirlineSafety (airline, avail_seat_km_per_week, incidents_85_99, fatal_accidents_85_99, fatalities_85_99, incidents_00_14, fatal_accidents_00_14, fatalities_00_14) VALUES ('New Airline', 1000000, 2, 0, 0, 1, 0, 0);
```

```sql
UPDATE AirlineSafety SET airline = 'Updated Airline A', avail_seat_km_per_week = 600000, incidents_85_99 = 3, fatal_accidents_85_99 = 0, fatalities_85_99 = 1, incidents_00_14 = 0, fatal_accidents_00_14 = 1, fatalities_00_14 = 0 WHERE id = 1;
```

```sql
DELETE FROM AirlineSafety WHERE id = 2;
```

