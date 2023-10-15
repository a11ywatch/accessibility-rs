# web-accessibility-rules

List of [WCAG2.1 techniques](https://www.w3.org/TR/WCAG21/) and whether or not we have it handled.

## WCAG 2.1

| Technique                                          | Description                                                                   | WCAG  | Type  | Name        | Complete |
| -------------------------------------------------- | ----------------------------------------------------------------------------- | ----- | ----- | ----------- | -------- |
| [H25](https://www.w3.org/TR/WCAG20-TECHS/H25.html) | empty titles                                                                  | A-AAA | error |             | ✅       |
| [H30](https://www.w3.org/TR/WCAG20-TECHS/H30.html) | text alternative img                                                          | A-AAA | error |             | ✅       |
| [H32](https://www.w3.org/TR/WCAG20-TECHS/H32.html) | missing form submit button                                                    | A-AAA | error |             | ✅       |
| [H36](https://www.w3.org/TR/WCAG20-TECHS/H36.html) | missing form img alt                                                          | A-AAA | error |             | ✅       |
| [H37](https://www.w3.org/TR/WCAG20-TECHS/H37.html) | missing img alt                                                               | A-AAA | error |             | ✅       |
| [H42](https://www.w3.org/TR/WCAG20-TECHS/H42.html) | heading found with no content                                                 | A-AAA | error |             | ✅       |
| [H57](https://www.w3.org/TR/WCAG20-TECHS/H57.html) | html contains valid lang                                                      | A-AAA | error |             | ✅       |
| [H57](https://www.w3.org/TR/WCAG20-TECHS/H57.html) | lang attribute of the document element does not appear to be well-formed      | A-AAA | error | 3.Lang      | ✅       |
| [H57](https://www.w3.org/TR/WCAG20-TECHS/H57.html) | xml:lang attribute of the document element does not appear to be well-formed. | A-AAA | error | 3.XmlLang   | ✅       |
| [H64](https://www.w3.org/TR/WCAG20-TECHS/H64.html) | iframe missing title attribute                                                | A-AAA | error | 1           | ✅       |
| [H91](https://www.w3.org/TR/WCAG20-TECHS/H91.html) | anchor valid href attribute, but no link content                              | A-AAA | error | A.NoContent | ✅       |
| [H91](https://www.w3.org/TR/WCAG20-TECHS/H91.html) | anchor found but no link content                                              | A-AAA | error | A.EmptyNoId | ✅       |
| [F40](https://www.w3.org/TR/WCAG20-TECHS/F40.html) | meta redirect used with a time limit                                          | A-AAA | error | 2           | ✅       |
| [F41](https://www.w3.org/TR/WCAG20-TECHS/F41.html) | meta refresh used to reload the page                                          | A-AAA | error | 2           | ✅       |
| [F47](https://www.w3.org/TR/WCAG20-TECHS/F47.html) | blink element used for attention                                              | A-AAA | error |             | ✅       |
| [F77](https://www.w3.org/TR/WCAG20-TECHS/F77.html) | duplicate ID found                                                            | A-AAA | error |             | ✅       |

Errors that can be to be tested with automation `16/70`.

Key: ✅ = Complete, ✔️ = Complete with a bit of missing details.
