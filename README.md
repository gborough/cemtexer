# Cemtexer

A utility for generating and validating Australia Banking Association
Cemtex file(.aba), currently featuring a command line interface written
in the Rust programming language. Specifications for Cemtex file format
can be found in vairous sources such as this:
https://idoc.pub/documents/aba-file-format-details-online-cemtex-aba-file-conversion-csv-to-aba-d49o32yg8649

For enterprise and individual usages it is designed with the following in mind:

* **Royalty Free**: Cemtexer is free to use with constant feature enhancements,
an ideal alternative to mostly paid commercial products.

* **Self Integration**: Cemtexer allows users to easily self integrate by using
commonly agreed template format.

* **Fast and Safe**: Cemtexer leverages Rust programming language for its
unique covenant regarding performance and memory safety.

* **Multipurpose**: Cemtexer enables enterprise users for batch processing
of Cemtex file in a scripted fashion and targets individual with an easy to use
UI in the upcoming releases.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/tokio.svg
[crates-url]: https://crates.io/crates/cemtexer
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/gborough/cemtexer/blob/main/LICENSE

## How to Use

Download the latest release from:
https://github.com/gborough/cemtexer/releases/download/v0.1.0/cemtexer

The command line interface currently features four functions, type cemtexer -h
for command line options

* Display an example template to stdout for integration instruction
Example:

```
cemtexer showtemplate
```

* Generate a template at a user designated location to be fill in for settlement
Example:

```
cemtexer gentemplate /path/to/template
```

* Generate the .aba file at a user designated location from the template file and 
the user generated .csv settlement file
Example:

```
cemtexer abagen --template /path/to/template \
--csv /path/to/csv.csv \
--aba /path/to/aba.aba
```

* Validate existing .aba file from a user designated location and generate
a report to a user designated location
Example:

```
cemtexer abacheck --aba /path/to/aba.aba \
--report /path/to/report
```

## Self Integration Guide

In order to seemlessly self integrate and run program in an automated fashion,
it is recommended that users abide by the following file format conventions. They
are designed to cover absolute majority of settlement cases:

* For displaying a example settlement template file, simply run:

```
cemtexer showtemplate
```

then proceed generate and populate the a template file by following the requirements
shown above.

* For .csv which is usually generated by your account software, the following format
must be followed rigidly, comma separated, all fields mantory except for comments and
tax witholdings.

```
BSB,Account Number,Account Name,Amount,Optional Comments,Optional Tax Withholding
```
Note: The Amount field must be in either cent-denoted format(period free: e.g. 123) or
two-decimal format(e.g. 123.45), with or without dollar sign prefix, but should not be
a mixture of both. See notes below. 

Example of a fully filled entry:

```
063-000,1234567,Alice Smith,37.00,Purchase,0.37
```

Example of optional fields:

```
063-000,1234567,Alice Smith,$37.00,,0.37
063-001,9876543,Bob Smith,58.00,Purchase,
063-002,1029384,Eve Smith,$10.00,,
```

## Notes

* APCA number validation is not supporte in the current release due to lack of data source,
please let me know if there is a reliable data source that I can use.

* Merge multiple payment is not available yet as most accounting softwares handle
this feature already, also it is unclear how certain fields are to be merged due to
different requirements by banks and enterprises. Please let me know if there is a 
strong case for enabling this feature and I shall look into it.

* If the amount field in the csv field is a mixture of two-decimal format and cent-denoted
format(extremely rare but not unheard of), we cannot guarantee the correctness as no validation
is put in place because we cannot surmise the user's real intention.

* Error reporting for deserialising .csv files is generic for non-compliant format, namely
missing period(s) for field(s), hence it is crucial that integration guide should be followed
verbatim. Overall we assume that the accounting software would output the correct format.

* Due to Cemtex using non-compliant date format(DDMMYY), the leap year validation will only
run up to year 9999 A.D.

## TODOS

* Add Nix build files. In progress.

* Add cross compile script. In progress.

## Future Releases

* A simple UI for individual users(accountants I would assume) which is compiled to a wasm
target for easy distribution and fully self contained. Planned, in progress.

* A free to use web based SAAS solution encompassing more accounting functions. Planned.

## License

This project is licensed under the [MIT license].

[MIT license]: https://

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Cemtexer by you, shall be licensed as MIT, without any additional
terms or conditions.