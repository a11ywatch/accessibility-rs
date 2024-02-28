# web-accessibility-rules

List of [WCAG2.1 techniques](https://www.w3.org/TR/WCAG21/) and whether or not we have it handled.

## WCAG 2.1

| Technique                                          | Description                                                                  | WCAG  | Type  | Name            | Complete |
| -------------------------------------------------- | ---------------------------------------------------------------------------- | ----- | ----- | --------------- | -------- |
| [G18](https://www.w3.org/TR/WCAG20-TECHS/G18.html) | element has insufficient contrast at this conformance level                  | AAA   | error |                 |          |
| [H2](https://www.w3.org/TR/WCAG20-TECHS/H2.html)   | img element in link has alt text that duplicates the text content of link    | A-AAA | error | EG              | ✅       |
| [H25](https://www.w3.org/TR/WCAG20-TECHS/H25.html) | empty titles                                                                 | A-AAA | error |                 | ✅       |
| [H30](https://www.w3.org/TR/WCAG20-TECHS/H30.html) | text alternative img                                                         | A-AAA | error |                 | ✅       |
| [H32](https://www.w3.org/TR/WCAG20-TECHS/H32.html) | missing form submit button                                                   | A-AAA | error |                 | ✅       |
| [H35](https://www.w3.org/TR/WCAG20-TECHS/H35.html) | applet without body                                                          | A-AAA | error | 2               | ✅       |
| [H35](https://www.w3.org/TR/WCAG20-TECHS/H35.html) | applet without alt text                                                      | A-AAA | error | 3               | ✅       |
| [H36](https://www.w3.org/TR/WCAG20-TECHS/H36.html) | missing form img alt                                                         | A-AAA | error |                 | ✅       |
| [H37](https://www.w3.org/TR/WCAG20-TECHS/H37.html) | missing img alt                                                              | A-AAA | error |                 | ✅       |
| [H42](https://www.w3.org/TR/WCAG20-TECHS/H42.html) | heading found with no content                                                | A-AAA | error |                 | ✅       |
| [H44](https://www.w3.org/TR/WCAG20-TECHS/H44.html) | label's "for" attribute contains an ID that does not exist                   | A-AAA | error |                 | ✅       |
| [H53](https://www.w3.org/TR/WCAG20-TECHS/H53.html) | object elements must contain text alternative                                | A-AAA | error |                 | ✅       |
| [H57](https://www.w3.org/TR/WCAG20-TECHS/H57.html) | html contains valid lang                                                     | A-AAA | error |                 | ✅       |
| [H57](https://www.w3.org/TR/WCAG20-TECHS/H57.html) | lang attribute of the document element does not appear to be well-formed     | A-AAA | error | 3.Lang          | ✅       |
| [H57](https://www.w3.org/TR/WCAG20-TECHS/H57.html) | xml:lang attribute of the document element does not appear to be well-formed | A-AAA | error | 3.XmlLang       | ✅       |
| [H64](https://www.w3.org/TR/WCAG20-TECHS/H64.html) | iframe missing title attribute                                               | A-AAA | error | 1               | ✅       |
| [H67](https://www.w3.org/TR/WCAG20-TECHS/H67.html) | Img element with empty alt text must have absent or empty title attribute    | A-AAA | error | 1               | ✅       |
| [H71](https://www.w3.org/TR/WCAG20-TECHS/H71.html) | fieldset missing legend element                                              | A-AAA | error | 2               | ✅       |
| [H91](https://www.w3.org/TR/WCAG20-TECHS/H91.html) | anchor valid href attribute, but no link content                             | A-AAA | error | A.NoContent     | ✅       |
| [H91](https://www.w3.org/TR/WCAG20-TECHS/H91.html) | anchor found but no link content                                             | A-AAA | error | A.EmptyNoId     | ✅       |
| [H91](https://www.w3.org/TR/WCAG20-TECHS/H91.html) | form control needs name                                                      | A-AAA | error | [NodeName].Name | ✔️        |
| [H93](https://www.w3.org/TR/WCAG20-TECHS/H93.html) | label has multiple for ids                                                   | A-AAA | error |                 | ✅       |
| [F40](https://www.w3.org/TR/WCAG20-TECHS/F40.html) | meta redirect used with a time limit                                         | A-AAA | error | 2               | ✅       |
| [F41](https://www.w3.org/TR/WCAG20-TECHS/F41.html) | meta refresh used to reload the page                                         | A-AAA | error | 2               | ✅       |
| [F47](https://www.w3.org/TR/WCAG20-TECHS/F47.html) | blink element used for attention                                             | A-AAA | error |                 | ✅       |

Errors that can be to be tested with automation `25/70`.

Key: ✅ = Complete, ✔️ = Complete with a bit of missing details.
