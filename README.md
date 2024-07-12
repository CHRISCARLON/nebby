# Excel Quick Scan

## WORK IN PROGRESS

A simple CLI tool named Excel Quick Scan (ExQS).

This is a very basic tool that provides basic information about remote excel files.

This is a project for me to practice Rust but I may make it available as a package in the future.

## It currently only has 2 basic commands:

```zsh
exqs basic --url "URL to Excel File here"
```

```zsh
exqs format --url "URL to Excel File here"
```

## Example Output for exqs basic:

```text
Sheet Name: 2023
Total number of columns: 7
Total number of rows: 76
┌───────────────────────────────────────────────┬───────────┐
│ Column Headers                                ┆ Data Type │
╞═══════════════════════════════════════════════╪═══════════╡
│ Column 1: GENDER                              ┆ String    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 2: POSTCODE                            ┆ String    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 3: DOD                                 ┆ Unknown   │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 4: AGE                                 ┆ Float     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 5: FUNERAL COST                        ┆ Float     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 6: COSTS RECOVERED                     ┆ Float     │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Column 7: DATE REFERRED TO TREASURY SOLICITOR ┆ String    │
└───────────────────────────────────────────────┴───────────┘
Data Row 1: MALE LS25 41619 72 1590 1590 41900
Data Row 2: MALE LS22 41625 69 1350.54 1350.54 NOT REFERRED
Data Row 3: MALE LS8 41628 77 1702 0 NOT REFERRED
Data Row 4: MALE LS11 41653 63 2270 1673.4 NOT REFERRED
Data Row 5: MALE LS6 41654 54 1307 0 NOT REFERRED
Data Row 6: FEMALE LS12 41655 91 1474 1474 41767
Data Row 7: MALE LS14 41658 89 1630 1630 NOT REFERRED
Data Row 8: MALE LS12 41673 66 1378.5 1378.5 NOT REFERRED
Data Row 9: MALE LS3 41684 56 1660.64 855.49 NOT REFERRED
Data Row 10: FEMALE LS9 41689 46 1266 215.24 NOT REFERRED
```

## Example Output for exqs format:

```text
Analysing sheet: Cover sheet
Formatting issues:
- No header row found at all
- Sheet contains 3 merged region(s)
Analysing sheet: Instructions
Formatting issues:
- The header is not on the first row. Found on row 12
- Sheet contains 11 merged region(s)
Analysing sheet: Overview
Formatting issues:
- The header is not on the first row. Found on row 4
Analysing sheet: Resilience
Formatting issues:
- The header is not on the first row. Found on row 6
- Sheet contains 1 merged region(s)
Analysing sheet: Buildings and Energy
Formatting issues:
- The header is not on the first row. Found on row 6
- Sheet contains 1 merged region(s)
Analysing sheet: Transport
Formatting issues:
- The header is not on the first row. Found on row 6
- Sheet contains 1 merged region(s)
Analysing sheet: Waste
Formatting issues:
- The header is not on the first row. Found on row 6
- Sheet contains 1 merged region(s)
Analysing sheet: People
Formatting issues:
- The header is not on the first row. Found on row 6
- Sheet contains 1 merged region(s)
Analysing sheet: EqIA
Formatting issues:
- The header is not on the first row. Found on row 3
Analysing sheet: Results
Formatting issues:
- The header is not on the first row. Found on row 7
- Sheet contains 1 merged region(s)
Analysing sheet: Guidance
No formatting issues found.
Analysing sheet: Lists
No formatting issues found.
Analysing sheet: Background lookup tables
No formatting issues found.
Successfully Processed formatting!
```
