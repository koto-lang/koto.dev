export function register_koto_mode() {
  ace.define(
    "ace/mode/koto",
    ["require", "exports", "ace/lib/oop", "ace/mode/text"],
    (acequire, exports) => {
      const oop = acequire("ace/lib/oop");
      const TextMode = acequire("ace/mode/text").Mode;

      var TextHighlightRules = acequire("ace/mode/text_highlight_rules")
        .TextHighlightRules;

      var KotoHighlightRules = function () {
        this.$rules = {
          start: [
            {
              include: "#comment-block",
            },
            {
              token: "keyword.comment.line.koto",
              regex: /#.*/,
            },
            {
              include: "#keywords",
            },
            {
              include: "#numbers",
            },
            {
              include: "#operators",
            },
            {
              include: "#punctuation",
            },
            {
              include: "#strings",
            },
            {
              include: "#identifiers",
            },
          ],
          "#comment-block": [
            {
              token: "comment.block.koto",
              regex: /#-/,
              push: [
                {
                  token: "comment.block.koto",
                  regex: /-#/,
                  next: "pop",
                },
                {
                  token: "constant.character.escape.koto",
                  regex: /\\./,
                },
                {
                  include: "#comment-block",
                },
                {
                  defaultToken: "comment.block.koto",
                },
              ],
            },
          ],
          "#identifiers": [
            {
              token: "entity.name.function.koto",
              regex: /\b[[:alpha:]_][[:alnum:]_]*(?=\:)\b/,
            },
            // {
            //   token: "entity.name.function.koto",
            //   regex: /\b(?!\.\.)(?<=\.)[[:alpha:]_][[:alnum:]_]*\b/,
            // },
          ],
          "#keywords": [
            {
              token: "constant.language.koto",
              regex: /\b(?:false|true)\b/,
            },
            {
              token: "constant.language.self.koto",
              regex: /\bself\b/,
            },
            {
              token: "support.function.koto",
              regex: /\b(?:assert|assert_eq|assert_ne|assert_near)\b/,
            },
            {
              token: "keyword.control.koto",
              regex: /\b(?:catch|finally|for|in|loop|return|throw|try|until|while|yield)\b/,
            },
            {
              token: "keyword.control.conditional.koto",
              regex: /\b(?:else|if|match|switch|then)\b/,
            },
            {
              token: "keyword.control.import.koto",
              regex: /\b(?:export|from|import)\b/,
            },
          ],
          "#numbers": [
            {
              token: "constant.numeric.koto",
              regex: /\b-?[0-9]+\b/,
            },
            {
              token: "constant.numeric.koto",
              regex: /\b-?[0-9]+.?[0-9]+(?:e[-+]?[0-9]+)?\b/,
            },
            {
              token: "constant.numeric.koto",
              regex: /\b-?0b[01]+\b/,
            },
            {
              token: "constant.numeric.koto",
              regex: /\b-?0o[0-7]+\b/,
            },
            {
              token: "constant.numeric.koto",
              regex: /\b-?0x[0-9a-fA-F]+\b/,
            },
          ],
          "#operators": [
            {
              token: "keyword.operator.koto",
              regex: /\b(?:and|not|or)\b/,
            },
            {
              token: "keyword.operator.koto",
              regex: /\+|-|%|\*|\//,
            },
            {
              token: "keyword.operator.koto",
              regex: /\+=|-=|\*=|\/=|%=/,
            },
            {
              token: "keyword.operator.koto",
              regex: /==?|<=?|>=?/,
            },
            {
              token: "keyword.operator.koto",
              regex: /\.\.=?/,
            },
          ],
          "#punctuation": [
            {
              token: "punctuation.brackets.curly.koto",
              regex: /\{|\}/,
            },
            {
              token: "punctuation.brackets.round.koto",
              regex: /\(|\)/,
            },
            {
              token: "punctuation.brackets.square.koto",
              regex: /\[|\]/,
            },
            {
              token: "punctuation.comma.koto",
              regex: /,/,
            },
            {
              token: "punctuation.definition.parameters.koto",
              regex: /\|/,
            },
            {
              token: "punctuation.meta.decorator.koto",
              regex: /@/,
            },
          ],
          "#strings": [
            {
              token: "string.quoted.double.koto",
              regex: /"/,
              push: [
                {
                  token: "string.quoted.double.koto",
                  regex: /"/,
                  next: "pop",
                },
                {
                  token: "constant.character.escape.koto",
                  regex: /\\./,
                },
                {
                  defaultToken: "string.quoted.double.koto",
                },
              ],
            },
          ],
        };

        this.normalizeRules();
      };

      KotoHighlightRules.metaData = {
        $schema:
          "https://raw.githubusercontent.com/martinring/tmlanguage/master/tmlanguage.json",
        name: "Koto",
        scopeName: "source.koto",
      };

      oop.inherits(KotoHighlightRules, TextHighlightRules);
      var Mode = function () {
        this.HighlightRules = KotoHighlightRules;
      };
      oop.inherits(Mode, TextMode);
      exports.Mode = Mode;
    }
  );
}
