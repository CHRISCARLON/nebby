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

```zsh
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
