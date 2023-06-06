# badgestore-update-badge-action

![Lines of code](https://api.badgestore.dev/badge/a5d8648096584f2f/local)

Update a badge on [badgestore.dev](https://badgestore.dev) (or self-hosted instance)

Generated based on [dbanty/rust-github-action-template](https://github.com/dbanty/rust-github-action-template)

## Inputs
| Name             | Description                                        | Default                      |
|------------------|----------------------------------------------------|------------------------------|
| `left-label`     | The left label of the badge                        | `<none>`                     |
| `right-label`    | The right label of the badge                       | `<none>`                     |
| `left-color`     | Hex color                                          | `<none>`                     |
| `right-color`    | Hex color                                          | `<none>`                     |
| `right-color`    | Hex color                                          | `<none>`                     |
| `read-write-key` | Read and Write keys in format "read_key:write_key" | `<none>`                     |
| `api-url`        | The api url of the badgestore                      | `https://api.badgestore.dev` |

Examples:
- `left-label`:
  - `Lines of code`
- `right-label`
  - `-100`
- `left-color`
  - `555555`
- `right-color`
  - `999999`

## Outputs
| Name        |
|-------------|
| left-label  |
| right-label |
| left-color  |
| right-color |
| read-key    |

## Example usage
```yaml
name: Count loc, and update to badgestore
on: [push]
jobs:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v3
    - id: loc
      name: Count lines of code
      uses: Sh1nku/count-loc-action@v1
      with:
        excluded: "*.json,*.yaml"
    - uses: Sh1nku/badgestore-update-badge-action@v1
      name: Update badge
      id: badge
      with:
        right-label: ${{ steps.loc.outputs.Total_code }}
        read-write-key: ${{ secrets.ACTION_RW_KEY }}
    - name: Verify content changed
      if: steps.badge.outputs.right-label != steps.loc.outputs.Total_code
      run: echo "The output of the badge was not equal to the input ${{ steps.badge.outputs.right-label}} - ${{ steps.loc.outputs.Total_code}}" && exit 1
```