# Excel Quick Scan

## WORK IN PROGRESS 

A simple CLI tool name Excel Quick Scan.  

This is a very basic tool, for forgetful people like me, that provides basic information about excel files - both remote and local. 

This is a project for me to practice Rust but I may make it available as a package in the future.  

## It currently only has 2 basic commands: 

```zsh 
exqs --file file/path/here
```

```zsh
exqs --url "URL HERE"
```

## Example Output:

```zsh
exqs --url "https://data.london.gov.uk/download/mps-monthly-crime-dahboard-data/7f45d2fe-bf69-4395-b814-cadd5ec48489/M1045_MonthlyCrimeDashboard_TNOCrimeData_202406.xlsx"
Sheet Name: MPS_MonthlyCrimeDashboard_TNOCr
Total number of columns: 12
Total number of rows: 223821
┌────────────────────────────┬───────────┐
│ Column Headers             ┆ Data Type │
╞════════════════════════════╪═══════════╡
│ Column 1: Month_Year       ┆ String    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 2: Area Type        ┆ String    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 3: Borough_SNT      ┆ String    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 4: Area name        ┆ String    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 5: Area code        ┆ String    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 6: Offence Group    ┆ String    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 7: Offence Subgroup ┆ String    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 8: Measure          ┆ String    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 9: Financial Year   ┆ String    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 10: FY_FYIndex      ┆ String    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 11: Count           ┆ String    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 12: Refresh Date    ┆ String    │
└────────────────────────────┴───────────┘
```
