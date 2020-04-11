{{#each data}}
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct {{name}} {
    {{#each fields}}{{name}}: Vec<{{this.type}}>,
    {{/each}}{{#each others}}{{name}}: {{this.type}},
    {{/each}}
}

impl Message for {{name}} {
    fn from_bin(bin: Vec<u8>, n: usize) -> Self {
        let object_size = (0{{#each fields}} + {{bits}}{{/each}}) / 8;
        {{#if fields}}
        // One vector for each type inside objects
        {{#each fields}}let mut {{name}} = Vec::new();
        {{/each}}
        // For each object
        for i in 0..n {
            let offset = i * object_size;
            let mut acumulated = 0;
            {{#each fields}}
            // {{name}}: {{type}} ==============
            {{#if (eq type "u8")}}
            let obj_{{name}} = bin[offset + acumulated];
            acumulated += {{bits}} / 8;
            {{else}}
            let obj_{{name}} = {
                let mut tmp = 0;
                for i in acumulated..({{bits}}/8)+1 {
                    tmp += (bin[offset+i] as {{type}}) << ((i as {{type}})-acumulated as {{type}}) * 8{{type}};
                }
                tmp
            };
            {{/if}}
            {{name}}.push(obj_{{name}});
            {{/each}}
        }
        {{/if}}

        // OTHERS
        let others_offset = n * object_size;
        let mut acumulated = 0;
        {{#each others}}
        {{#if (eq type "TimeLabel")}}// {{name}}: {{type}}
        let {{name}} = TimeLabel::A(TimeLabelA::from_be_bytes([
            bin[acumulated+others_offset],
            bin[acumulated+others_offset+1],
            bin[acumulated+others_offset+2],
            bin[acumulated+others_offset+3],
            bin[acumulated+others_offset+4]
        ]));
        {{else}}
        let {{name}} = {
            let mut tmp: {{type}} = 0;
            for i in others_offset+acumulated..others_offset+acumulated+{{bits}}/8 {
                tmp += (bin[others_offset+i] as {{type}}) << ((i as {{type}})-acumulated as {{type}}) * 8{{type}};
            }
            tmp
        };
        {{/if}}
        acumulated += {{bits}} / 8;
        {{/each}}
        {{this.name}} {
            {{#each fields}}{{name}},
            {{/each}}
            {{#each others}}{{name}},
            {{/each}}
        }
    }

    fn into_bin(self) -> Vec<u8> {
        let bin = Vec::new();
        bin
    }
}
{{/each}}