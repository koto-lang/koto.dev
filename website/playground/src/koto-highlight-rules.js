export function register_koto_editor_mode() {
  ace.define(
    "ace/mode/koto",
    ["require", "exports", "ace/lib/oop", "ace/mode/text"],
    (acequire, exports) => {
      const oop = acequire("ace/lib/oop");
      const TextMode = acequire("ace/mode/text").Mode;

      var TextHighlightRules = acequire("ace/mode/text_highlight_rules")
        .TextHighlightRules;

      const identifier = "[a-zA-Z\\$_\u00a1-\uffff][a-zA-Z\\d\\$_\u00a1-\uffff]*";

      var KotoHighlightRules = function() {
        // regexp must not have capturing parentheses. Use (?:) instead.
        // regexps are ordered -> the first match is used

        this.$rules = {
          start: [{
            include: "#all"
          }],
          "#all": [{
            include: "#comment-block"
          }, {
            token: "keyword.comment.line.koto",
            regex: /#.*/
          }, {
            include: "#keyword"
          }, {
            include: "#number"
          }, {
            include: "#operator"
          }, {
            include: "#punctuation"
          }, {
            include: "#section"
          }, {
            include: "#string"
          }, {
            include: "#let"
          }, {
            include: "#function"
          }, {
            include: "#identifier"
          }],
          "#comment-block": [{
            token: "comment.block.koto",
            regex: /#-/,
            push: [{
              token: "comment.block.koto",
              regex: /-#/,
              next: "pop"
            }, {
              token: "constant.character.escape.koto",
              regex: /\\./
            }, {
              defaultToken: "comment.block.koto"
            }]
          }],
          "#identifier": [{
            token: "variable.other.member.koto",
            regex: "\\b" + identifier + "(?=\\:)\\b"
          }, {
            token: "text.koto",
            regex: identifier
          }],
          "#id_with_type": [{
            token: [
              "text.koto",
              "punctuation.koto",
              "storage.type.koto"
            ],
            regex: `(${identifier})(?:(\\s*:\\s*)(${identifier}\\??))?`,
          }],
          "#keyword": [{
            token: "constant.language.koto",
            regex: /\b(?:false|true|null)\b/
          }, {
            token: "constant.language.self.koto",
            regex: /\bself\b/
          }, {
            token: "support.function.koto",
            regex: /\b(?:assert|assert_eq|assert_ne|assert_near)\b/
          }, {
            token: "keyword.control.koto",
            regex: /\b(?:catch|finally|for|in|loop|throw|try|until|while)\b/
          }, {
            token: "keyword.control.flow.koto",
            regex: /\b(?:break|continue|return|yield)\b/
          }, {
            token: "keyword.control.conditional.koto",
            regex: /\b(?:else|if|match|switch|then)\b/
          }, {
            token: "keyword.control.import.koto",
            regex: /\b(?:export|from|import)\b/
          }, {
            token: "keyword.other",
            regex: /\bdebug\b/
          }],
          "#number": [{
            token: "constant.numeric.koto",
            regex: /\b-?[0-9]+\b/
          }, {
            token: "constant.numeric.koto",
            regex: /\b-?[0-9]+.?[0-9]+(?:e[-+]?[0-9]+)?\b/
          }, {
            token: "constant.numeric.koto",
            regex: /\b-?0b[01]+\b/
          }, {
            token: "constant.numeric.koto",
            regex: /\b-?0o[0-7]+\b/
          }, {
            token: "constant.numeric.koto",
            regex: /\b-?0x[0-9a-fA-F]+\b/
          }],
          "#operator": [{
            token: "keyword.operator.koto",
            regex: /\b(?:and|not|or)\b/
          }, {
            token: "keyword.operator.koto",
            regex: /\+|-|%|\*|\//
          }, {
            token: "keyword.operator.koto",
            regex: /\+=|-=|\*=|\/=|%=/
          }, {
            token: "keyword.operator.koto",
            regex: /==?|<=?|>=?/
          }, {
            token: "keyword.operator.koto",
            regex: /\.\.=?/
          }],
          "#punctuation": [{
            token: "punctuation.brackets.round.koto",
            regex: /\(|\)/
          }, {
            token: "punctuation.dot.koto",
            regex: /\./
          }, {
            token: "punctuation.comma.koto",
            regex: /,/
          }, {
            token: "punctuation.meta.decorator.koto",
            regex: "@(?:" + identifier + ")?"
          }],
          "#section": [{
            token: "punctuation.brackets.curly.koto",
            regex: /{/,
            push: [{
              token: "punctuation.brackets.curly.koto",
              regex: /}/,
              next: "pop"
            }, {
              include: "#all"
            }, {
              defaultToken: "punctuation.brackets.curly.koto"
            }]
          }, {
            token: "punctuation.brackets.square.koto",
            regex: /\[/,
            push: [{
              token: "punctuation.brackets.square.koto",
              regex: /]/,
              next: "pop"
            }, {
              include: "#all"
            }, {
              defaultToken: "punctuation.brackets.square.koto"
            }]
          }],
          "#string": [{
            include: "#string-single-quoted"
          }, {
            include: "#string-double-quoted"
          }],
          "#string-single-quoted": [{
            token: "string.quoted.single.koto",
            regex: /'/,
            push: [{
              token: "string.quoted.single.koto",
              regex: /'/,
              next: "pop"
            }, {
              include: "#string-escape"
            }, {
              include: "#string-template"
            }, {
              defaultToken: "string.quoted.single.koto"
            }]
          }],
          "#string-double-quoted": [{
            token: "string.quoted.double.koto",
            regex: /"/,
            push: [{
              token: "string.quoted.double.koto",
              regex: /"/,
              next: "pop"
            }, {
              include: "#string-escape"
            }, {
              include: "#string-template"
            }, {
              defaultToken: "string.quoted.double.koto"
            }]
          }],
          "#string-escape": [{
            token: "constant.character.escape.koto",
            regex: /\\['$rnt"\\]/
          }, {
            token: "constant.character.escape.koto",
            regex: /\\$/
          }, {
            token: "constant.character.escape.koto",
            regex: /\\x[0-9a-fA-F]{2}/
          }, {
            token: "constant.character.escape.koto",
            regex: /\\u{[0-9a-fA-F]{1,6}}/
          }],
          "#string-template": [{
            token: "variable.parameter.koto",
            regex: /{/,
            push: [{
              token: "variable.parameter.koto",
              regex: /}/,
              next: "pop"
            }, {
              include: "#all"
            }, {
              defaultToken: "variable.parameter.koto"
            }]
          }],
          "#let": [{
            token: "keyword.control.koto",
            regex: /\blet\b/,
            push: [{
              token: "keyword.operator.assignment.koto",
              regex: /\=/,
              next: "pop"
            }, {
              include: "#id_with_type"
            }]
          }],
          "#function": [{
            token: "keyword.other.function-definition.koto",
            regex: /\|/,
            push: [{
              token: "keyword.other.function-definition.koto",
              regex: /\|/,
              next: "pop"
            }, {
              include: "#function_arguments"
            }, {
              include: "#return_type"
            }]
          }],
          "#function_arguments": [
            {
              token: "punctuation.definition.arguments.begin.koto",
              regex: /\(/,
              push: [
                {
                  token: "punctuation.definition.arguments.end.koto",
                  regex: /\)/,
                  next: "pop"
                }, {
                  include: "#function_arguments"
                },
              ]
            },
            {
              token: "punctuation.separator.parameters.koto",
              regex: /,/
            },
            {
              include: "#id_with_type"
            }
          ],
          "#return_type": [{
            token: ["text", "storage.type.koto"],
            regex: /(->\s*)([[:alpha:]_][[:alnum:]_]*)/
          }]
        }

        this.normalizeRules();
      };

      KotoHighlightRules.metaData = {
        $schema:
          "https://raw.githubusercontent.com/martinring/tmlanguage/master/tmlanguage.json",
        name: "Koto",
        scopeName: "source.koto",
      };

      oop.inherits(KotoHighlightRules, TextHighlightRules);
      var Mode = function() {
        this.HighlightRules = KotoHighlightRules;
      };
      oop.inherits(Mode, TextMode);
      exports.Mode = Mode;
    }
  );
}
