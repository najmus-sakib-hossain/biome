// This file contains the list of all diagnostic categories for the Biome
// toolchain
//
// The `define_categories` macro is preprocessed in the build script for the
// crate in order to generate the static registry. The body of the macro
// consists of a list of key-value pairs defining the categories that have an
// associated hyperlink, then a list of string literals defining the remaining
// categories without a link.

// PLEASE, DON'T EDIT THIS FILE BY HAND.
// Use `just new-lintrule` to create a new rule.
// lint rules are lexicographically sorted and
// must be between `define_categories! {\n` and `\n    ;\n`.

define_categories! {
    "lint/a11y/noAccessKey": "https://biomejs.dev/linter/rules/no-access-key",
    "lint/a11y/noAriaHiddenOnFocusable": "https://biomejs.dev/linter/rules/no-aria-hidden-on-focusable",
    "lint/a11y/noAriaUnsupportedElements": "https://biomejs.dev/linter/rules/no-aria-unsupported-elements",
    "lint/a11y/noAutofocus": "https://biomejs.dev/linter/rules/no-autofocus",
    "lint/a11y/noDistractingElements": "https://biomejs.dev/linter/rules/no-distracting-elements",
    "lint/a11y/noHeaderScope": "https://biomejs.dev/linter/rules/no-header-scope",
    "lint/a11y/noInteractiveElementToNoninteractiveRole": "https://biomejs.dev/linter/rules/no-interactive-element-to-noninteractive-role",
    "lint/a11y/noLabelWithoutControl": "https://biomejs.dev/linter/rules/no-label-without-control",
    "lint/a11y/noNoninteractiveElementToInteractiveRole": "https://biomejs.dev/linter/rules/no-noninteractive-element-to-interactive-role",
    "lint/a11y/noNoninteractiveTabindex": "https://biomejs.dev/linter/rules/no-noninteractive-tabindex",
    "lint/a11y/noPositiveTabindex": "https://biomejs.dev/linter/rules/no-positive-tabindex",
    "lint/a11y/noRedundantAlt": "https://biomejs.dev/linter/rules/no-redundant-alt",
    "lint/a11y/noRedundantRoles": "https://biomejs.dev/linter/rules/no-redundant-roles",
    "lint/a11y/noStaticElementInteractions": "https://biomejs.dev/linter/rules/no-static-element-interactions",
    "lint/a11y/noSvgWithoutTitle": "https://biomejs.dev/linter/rules/no-svg-without-title",
    "lint/a11y/useAltText": "https://biomejs.dev/linter/rules/use-alt-text",
    "lint/a11y/useAnchorContent": "https://biomejs.dev/linter/rules/use-anchor-content",
    "lint/a11y/useAriaActivedescendantWithTabindex": "https://biomejs.dev/linter/rules/use-aria-activedescendant-with-tabindex",
    "lint/a11y/useAriaPropsForRole": "https://biomejs.dev/linter/rules/use-aria-props-for-role",
    "lint/a11y/useAriaPropsSupportedByRole": "https://biomejs.dev/linter/rules/use-aria-props-supported-by-role",
    "lint/a11y/useButtonType": "https://biomejs.dev/linter/rules/use-button-type",
    "lint/a11y/useFocusableInteractive": "https://biomejs.dev/linter/rules/use-focusable-interactive",
    "lint/a11y/useGenericFontNames": "https://biomejs.dev/linter/rules/use-generic-font-names",
    "lint/a11y/useHeadingContent": "https://biomejs.dev/linter/rules/use-heading-content",
    "lint/a11y/useHtmlLang": "https://biomejs.dev/linter/rules/use-html-lang",
    "lint/a11y/useIframeTitle": "https://biomejs.dev/linter/rules/use-iframe-title",
    "lint/a11y/useKeyWithClickEvents": "https://biomejs.dev/linter/rules/use-key-with-click-events",
    "lint/a11y/useKeyWithMouseEvents": "https://biomejs.dev/linter/rules/use-key-with-mouse-events",
    "lint/a11y/useMediaCaption": "https://biomejs.dev/linter/rules/use-media-caption",
    "lint/a11y/useSemanticElements": "https://biomejs.dev/linter/rules/use-semantic-elements",
    "lint/a11y/useValidAnchor": "https://biomejs.dev/linter/rules/use-valid-anchor",
    "lint/a11y/useValidAriaProps": "https://biomejs.dev/linter/rules/use-valid-aria-props",
    "lint/a11y/useValidAriaRole": "https://biomejs.dev/linter/rules/use-valid-aria-role",
    "lint/a11y/useValidAriaValues": "https://biomejs.dev/linter/rules/use-valid-aria-values",
    "lint/a11y/useValidAutocomplete": "https://biomejs.dev/linter/rules/use-valid-autocomplete",
    "lint/a11y/useValidLang": "https://biomejs.dev/linter/rules/use-valid-lang",
    "lint/complexity/noAdjacentSpacesInRegex": "https://biomejs.dev/linter/rules/no-adjacent-spaces-in-regex",
    "lint/complexity/noArguments": "https://biomejs.dev/linter/rules/no-arguments",
    "lint/complexity/noBannedTypes": "https://biomejs.dev/linter/rules/no-banned-types",
    "lint/complexity/noCommaOperator": "https://biomejs.dev/linter/rules/no-comma-operator",
    "lint/complexity/noEmptyTypeParameters": "https://biomejs.dev/linter/rules/no-empty-type-parameters",
    "lint/complexity/noExcessiveCognitiveComplexity": "https://biomejs.dev/linter/rules/no-excessive-cognitive-complexity",
    "lint/complexity/noExcessiveNestedTestSuites": "https://biomejs.dev/linter/rules/no-excessive-nested-test-suites",
    "lint/complexity/noExtraBooleanCast": "https://biomejs.dev/linter/rules/no-extra-boolean-cast",
    "lint/complexity/noFlatMapIdentity": "https://biomejs.dev/linter/rules/no-flat-map-identity",
    "lint/complexity/noForEach": "https://biomejs.dev/linter/rules/no-for-each",
    "lint/complexity/noStaticOnlyClass": "https://biomejs.dev/linter/rules/no-static-only-class",
    "lint/complexity/noThisInStatic": "https://biomejs.dev/linter/rules/no-this-in-static",
    "lint/complexity/noUselessCatch": "https://biomejs.dev/linter/rules/no-useless-catch",
    "lint/complexity/noUselessConstructor": "https://biomejs.dev/linter/rules/no-useless-constructor",
    "lint/complexity/noUselessContinue": "https://biomejs.dev/linter/rules/no-useless-continue",
    "lint/complexity/noUselessEmptyExport": "https://biomejs.dev/linter/rules/no-useless-empty-export",
    "lint/complexity/noUselessEscapeInRegex": "https://biomejs.dev/linter/rules/no-useless-escape-in-regex",
    "lint/complexity/noUselessFragments": "https://biomejs.dev/linter/rules/no-useless-fragments",
    "lint/complexity/noUselessLabel": "https://biomejs.dev/linter/rules/no-useless-label",
    "lint/complexity/noUselessLoneBlockStatements": "https://biomejs.dev/linter/rules/no-useless-lone-block-statements",
    "lint/complexity/noUselessRename": "https://biomejs.dev/linter/rules/no-useless-rename",
    "lint/complexity/noUselessStringConcat": "https://biomejs.dev/linter/rules/no-useless-string-concat",
    "lint/complexity/noUselessStringRaw": "https://biomejs.dev/linter/rules/no-useless-string-raw",
    "lint/complexity/noUselessSwitchCase": "https://biomejs.dev/linter/rules/no-useless-switch-case",
    "lint/complexity/noUselessTernary": "https://biomejs.dev/linter/rules/no-useless-ternary",
    "lint/complexity/noUselessThisAlias": "https://biomejs.dev/linter/rules/no-useless-this-alias",
    "lint/complexity/noUselessTypeConstraint": "https://biomejs.dev/linter/rules/no-useless-type-constraint",
    "lint/complexity/noUselessUndefinedInitialization": "https://biomejs.dev/linter/rules/no-useless-undefined-initialization",
    "lint/complexity/noVoid": "https://biomejs.dev/linter/rules/no-void",
    "lint/complexity/useArrowFunction": "https://biomejs.dev/linter/rules/use-arrow-function",
    "lint/complexity/useDateNow": "https://biomejs.dev/linter/rules/use-date-now",
    "lint/complexity/useFlatMap": "https://biomejs.dev/linter/rules/use-flat-map",
    "lint/complexity/useLiteralKeys": "https://biomejs.dev/linter/rules/use-literal-keys",
    "lint/complexity/useNumericLiterals": "https://biomejs.dev/linter/rules/use-numeric-literals",
    "lint/complexity/useOptionalChain": "https://biomejs.dev/linter/rules/use-optional-chain",
    "lint/complexity/useRegexLiterals": "https://biomejs.dev/linter/rules/use-regex-literals",
    "lint/complexity/useSimpleNumberKeys": "https://biomejs.dev/linter/rules/use-simple-number-keys",
    "lint/complexity/useSimplifiedLogicExpression": "https://biomejs.dev/linter/rules/use-simplified-logic-expression",
    "lint/complexity/useWhile": "https://biomejs.dev/linter/rules/use-while",
    "lint/correctness/noChildrenProp": "https://biomejs.dev/linter/rules/no-children-prop",
    "lint/correctness/noConstAssign": "https://biomejs.dev/linter/rules/no-const-assign",
    "lint/correctness/noConstantCondition": "https://biomejs.dev/linter/rules/no-constant-condition",
    "lint/correctness/noConstantMathMinMaxClamp": "https://biomejs.dev/linter/rules/no-constant-math-min-max-clamp",
    "lint/correctness/noConstructorReturn": "https://biomejs.dev/linter/rules/no-constructor-return",
    "lint/correctness/noEmptyCharacterClassInRegex": "https://biomejs.dev/linter/rules/no-empty-character-class-in-regex",
    "lint/correctness/noEmptyPattern": "https://biomejs.dev/linter/rules/no-empty-pattern",
    "lint/correctness/noGlobalObjectCalls": "https://biomejs.dev/linter/rules/no-global-object-calls",
    "lint/correctness/noInnerDeclarations": "https://biomejs.dev/linter/rules/no-inner-declarations",
    "lint/correctness/noInvalidBuiltinInstantiation": "https://biomejs.dev/linter/rules/no-invalid-builtin-instantiation",
    "lint/correctness/noInvalidConstructorSuper": "https://biomejs.dev/linter/rules/no-invalid-constructor-super",
    "lint/correctness/noInvalidDirectionInLinearGradient": "https://biomejs.dev/linter/rules/no-invalid-direction-in-linear-gradient",
    "lint/correctness/noInvalidGridAreas": "https://biomejs.dev/linter/rules/use-consistent-grid-areas",
    "lint/correctness/noInvalidNewBuiltin": "https://biomejs.dev/linter/rules/no-invalid-new-builtin",
    "lint/correctness/noInvalidPositionAtImportRule": "https://biomejs.dev/linter/rules/no-invalid-position-at-import-rule",
    "lint/correctness/noInvalidUseBeforeDeclaration": "https://biomejs.dev/linter/rules/no-invalid-use-before-declaration",
    "lint/correctness/noMissingVarFunction": "https://biomejs.dev/linter/rules/no-missing-var-function",
    "lint/correctness/noNewSymbol": "https://biomejs.dev/linter/rules/no-new-symbol",
    "lint/correctness/noNodejsModules": "https://biomejs.dev/linter/rules/no-nodejs-modules",
    "lint/correctness/noNonoctalDecimalEscape": "https://biomejs.dev/linter/rules/no-nonoctal-decimal-escape",
    "lint/correctness/noPrecisionLoss": "https://biomejs.dev/linter/rules/no-precision-loss",
    "lint/correctness/noPrivateImports": "https://biomejs.dev/linter/rules/no-private-imports",
    "lint/correctness/noRenderReturnValue": "https://biomejs.dev/linter/rules/no-render-return-value",
    "lint/correctness/noSelfAssign": "https://biomejs.dev/linter/rules/no-self-assign",
    "lint/correctness/noSetterReturn": "https://biomejs.dev/linter/rules/no-setter-return",
    "lint/correctness/noStringCaseMismatch": "https://biomejs.dev/linter/rules/no-string-case-mismatch",
    "lint/correctness/noSwitchDeclarations": "https://biomejs.dev/linter/rules/no-switch-declarations",
    "lint/correctness/noUndeclaredDependencies": "https://biomejs.dev/linter/rules/no-undeclared-dependencies",
    "lint/correctness/noUndeclaredVariables": "https://biomejs.dev/linter/rules/no-undeclared-variables",
    "lint/correctness/noUnknownFunction": "https://biomejs.dev/linter/rules/no-unknown-function",
    "lint/correctness/noUnknownMediaFeatureName": "https://biomejs.dev/linter/rules/no-unknown-media-feature-name",
    "lint/correctness/noUnknownProperty": "https://biomejs.dev/linter/rules/no-unknown-property",
    "lint/correctness/noUnknownPseudoClass": "https://biomejs.dev/linter/rules/no-unknown-pseudo-class",
    "lint/correctness/noUnknownPseudoClassSelector": "https://biomejs.dev/linter/rules/no-unknown-pseudo-class-selector",
    "lint/correctness/noUnknownPseudoElement": "https://biomejs.dev/linter/rules/no-unknown-selector-pseudo-element",
    "lint/correctness/noUnknownTypeSelector": "https://biomejs.dev/linter/rules/no-unknown-type-selector",
    "lint/correctness/noUnknownUnit": "https://biomejs.dev/linter/rules/no-unknown-unit",
    "lint/correctness/noUnmatchableAnbSelector": "https://biomejs.dev/linter/rules/no-unmatchable-anb-selector",
    "lint/correctness/noUnreachable": "https://biomejs.dev/linter/rules/no-unreachable",
    "lint/correctness/noUnreachableSuper": "https://biomejs.dev/docs/linter/rules/no-unreachable-super",
    "lint/correctness/noUnsafeFinally": "https://biomejs.dev/linter/rules/no-unsafe-finally",
    "lint/correctness/noUnsafeOptionalChaining": "https://biomejs.dev/linter/rules/no-unsafe-optional-chaining",
    "lint/correctness/noUnusedFunctionParameters": "https://biomejs.dev/linter/rules/no-unused-function-parameters",
    "lint/correctness/noUnusedImports": "https://biomejs.dev/linter/rules/no-unused-imports",
    "lint/correctness/noUnusedLabels": "https://biomejs.dev/linter/rules/no-unused-labels",
    "lint/correctness/noUnusedPrivateClassMembers": "https://biomejs.dev/linter/rules/no-unused-private-class-members",
    "lint/correctness/noUnusedVariables": "https://biomejs.dev/linter/rules/no-unused-variables",
    "lint/correctness/noVoidElementsWithChildren": "https://biomejs.dev/linter/rules/no-void-elements-with-children",
    "lint/correctness/noVoidTypeReturn": "https://biomejs.dev/linter/rules/no-void-type-return",
    "lint/correctness/useExhaustiveDependencies": "https://biomejs.dev/linter/rules/use-exhaustive-dependencies",
    "lint/correctness/useHookAtTopLevel": "https://biomejs.dev/linter/rules/use-hook-at-top-level",
    "lint/correctness/useImportExtensions": "https://biomejs.dev/linter/rules/use-import-extensions",
    "lint/correctness/useIsNan": "https://biomejs.dev/linter/rules/use-is-nan",
    "lint/correctness/useJsxKeyInIterable": "https://biomejs.dev/linter/rules/use-jsx-key-in-iterable",
    "lint/correctness/useValidForDirection": "https://biomejs.dev/linter/rules/use-valid-for-direction",
    "lint/correctness/useValidTypeof": "https://biomejs.dev/linter/rules/use-valid-typeof",
    "lint/correctness/useYield": "https://biomejs.dev/linter/rules/use-yield",
    "lint/nursery/colorNoInvalidHex": "https://biomejs.dev/linter/rules/color-no-invalid-hex",
    "lint/nursery/noAwaitInLoop": "https://biomejs.dev/linter/rules/no-await-in-loop",
    "lint/nursery/noBitwiseOperators": "https://biomejs.dev/linter/rules/no-bitwise-operators",
    "lint/nursery/noColorInvalidHex": "https://biomejs.dev/linter/rules/no-color-invalid-hex",
    "lint/nursery/noConsole": "https://biomejs.dev/linter/rules/no-console",
    "lint/nursery/noConstantBinaryExpression": "https://biomejs.dev/linter/rules/no-constant-binary-expression",
    "lint/nursery/noDestructuredProps": "https://biomejs.dev/linter/rules/no-destructured-props",
    "lint/nursery/noDoneCallback": "https://biomejs.dev/linter/rules/no-done-callback",
    "lint/nursery/noDuplicateAtImportRules": "https://biomejs.dev/linter/rules/no-duplicate-at-import-rules",
    "lint/nursery/noExcessiveLinesPerFunction": "https://biomejs.dev/linter/rules/no-excessive-lines-per-function",
    "lint/nursery/noFloatingPromises": "https://biomejs.dev/linter/rules/no-floating-promises",
    "lint/nursery/noGlobalDirnameFilename": "https://biomejs.dev/linter/rules/no-global-dirname-filename",
    "lint/nursery/noImplicitCoercion": "https://biomejs.dev/linter/rules/no-implicit-coercion",
    "lint/nursery/noImportCycles": "https://biomejs.dev/linter/rules/no-import-cycles",
    "lint/nursery/noImportantInKeyframe": "https://biomejs.dev/linter/rules/no-important-in-keyframe",
    "lint/nursery/noImportantStyles": "https://biomejs.dev/linter/rules/no-important-styles",
    "lint/nursery/noInvalidDirectionInLinearGradient": "https://biomejs.dev/linter/rules/no-invalid-direction-in-linear-gradient",
    "lint/nursery/noInvalidGridAreas": "https://biomejs.dev/linter/rules/use-consistent-grid-areas",
    "lint/nursery/noInvalidPositionAtImportRule": "https://biomejs.dev/linter/rules/no-invalid-position-at-import-rule",
    "lint/nursery/noMagicNumbers": "https://biomejs.dev/linter/rules/no-magic-numbers",
    "lint/nursery/noMissingGenericFamilyKeyword": "https://biomejs.dev/linter/rules/no-missing-generic-family-keyword",
    "lint/nursery/noMisusedPromises": "https://biomejs.dev/linter/rules/no-misused-promises",
    "lint/nursery/noNestedComponentDefinitions": "https://biomejs.dev/linter/rules/no-nested-component-definitions",
    "lint/nursery/noNoninteractiveElementInteractions": "https://biomejs.dev/linter/rules/no-noninteractive-element-interactions",
    "lint/nursery/noProcessGlobal": "https://biomejs.dev/linter/rules/no-process-global",
    "lint/nursery/noReactPropAssign": "https://biomejs.dev/linter/rules/no-react-prop-assign",
    "lint/nursery/noReactSpecificProps": "https://biomejs.dev/linter/rules/no-react-specific-props",
    "lint/nursery/noRestrictedElements": "https://biomejs.dev/linter/rules/no-restricted-elements",
    "lint/nursery/noSecrets": "https://biomejs.dev/linter/rules/no-secrets",
    "lint/nursery/noShadow": "https://biomejs.dev/linter/rules/no-shadow",
    "lint/nursery/noShorthandPropertyOverrides": "https://biomejs.dev/linter/rules/no-shorthand-property-overrides",
    "lint/nursery/noTsIgnore": "https://biomejs.dev/linter/rules/no-ts-ignore",
    "lint/nursery/noUnassignedVariables": "https://biomejs.dev/linter/rules/no-unassigned-variables",
    "lint/nursery/noUndeclaredDependencies": "https://biomejs.dev/linter/rules/no-undeclared-dependencies",
    "lint/nursery/noUnknownAtRule": "https://biomejs.dev/linter/rules/no-unknown-at-rule",
    "lint/nursery/noUnknownFunction": "https://biomejs.dev/linter/rules/no-unknown-function",
    "lint/nursery/noUnknownMediaFeatureName": "https://biomejs.dev/linter/rules/no-unknown-media-feature-name",
    "lint/nursery/noUnknownProperty": "https://biomejs.dev/linter/rules/no-unknown-property",
    "lint/nursery/noUnknownSelectorPseudoElement": "https://biomejs.dev/linter/rules/no-unknown-selector-pseudo-element",
    "lint/nursery/noUnknownUnit": "https://biomejs.dev/linter/rules/no-unknown-unit",
    "lint/nursery/noUnmatchableAnbSelector": "https://biomejs.dev/linter/rules/no-unmatchable-anb-selector",
    "lint/nursery/noUnresolvedImports": "https://biomejs.dev/linter/rules/no-unresolved-imports",
    "lint/nursery/noUnusedFunctionParameters": "https://biomejs.dev/linter/rules/no-unused-function-parameters",
    "lint/nursery/noUnwantedPolyfillio": "https://biomejs.dev/linter/rules/no-unwanted-polyfillio",
    "lint/nursery/noUselessBackrefInRegex": "https://biomejs.dev/linter/rules/no-useless-backref-in-regex",
    "lint/nursery/noUselessEscapeInString": "https://biomejs.dev/linter/rules/no-useless-escape-in-string",
    "lint/nursery/noUselessUndefined": "https://biomejs.dev/linter/rules/no-useless-undefined",
    "lint/nursery/noVueReservedProps": "https://biomejs.dev/linter/rules/no-vue-reserved-props",
    "lint/nursery/useAdjacentGetterSetter": "https://biomejs.dev/linter/rules/use-adjacent-getter-setter",
    "lint/nursery/useBiomeSuppressionComment": "https://biomejs.dev/linter/rules/use-biome-suppression-comment",
    "lint/nursery/useConsistentObjectDefinition": "https://biomejs.dev/linter/rules/use-consistent-object-definition",
    "lint/nursery/useConsistentResponse": "https://biomejs.dev/linter/rules/use-consistent-response",
    "lint/nursery/useExhaustiveSwitchCases": "https://biomejs.dev/linter/rules/use-exhaustive-switch-cases",
    "lint/nursery/useExplicitFunctionReturnType": "https://biomejs.dev/linter/rules/use-explicit-type",
    "lint/nursery/useExplicitType": "https://biomejs.dev/linter/rules/use-explicit-type",
    "lint/nursery/useExportsLast": "https://biomejs.dev/linter/rules/use-exports-last",
    "lint/nursery/useForComponent": "https://biomejs.dev/linter/rules/use-for-component",
    "lint/nursery/useGoogleFontPreconnect": "https://biomejs.dev/linter/rules/use-google-font-preconnect",
    "lint/nursery/useImportRestrictions": "https://biomejs.dev/linter/rules/use-import-restrictions",
    "lint/nursery/useIndexOf": "https://biomejs.dev/linter/rules/use-index-of",
    "lint/nursery/useIterableCallbackReturn": "https://biomejs.dev/linter/rules/use-iterable-callback-return",
    "lint/nursery/useJsonImportAttribute": "https://biomejs.dev/linter/rules/use-json-import-attribute",
    "lint/nursery/useJsxCurlyBraceConvention": "https://biomejs.dev/linter/rules/use-jsx-curly-brace-convention",
    "lint/nursery/useNamedOperation": "https://biomejs.dev/linter/rules/use-named-operation",
    "lint/nursery/useNamingConvention": "https://biomejs.dev/linter/rules/use-naming-convention",
    "lint/nursery/useNumericSeparators": "https://biomejs.dev/linter/rules/use-numeric-separators",
    "lint/nursery/useObjectSpread": "https://biomejs.dev/linter/rules/use-object-spread",
    "lint/nursery/useParseIntRadix": "https://biomejs.dev/linter/rules/use-parse-int-radix",
    "lint/nursery/useReadonlyClassProperties": "https://biomejs.dev/linter/rules/use-readonly-class-properties",
    "lint/nursery/useSingleJsDocAsterisk": "https://biomejs.dev/linter/rules/use-single-js-doc-asterisk",
    "lint/nursery/useSortedClasses": "https://biomejs.dev/linter/rules/use-sorted-classes",
    "lint/nursery/useSortedProperties": "https://biomejs.dev/linter/rules/use-sorted-properties",
    "lint/nursery/useSymbolDescription": "https://biomejs.dev/linter/rules/use-symbol-description",
    "lint/nursery/useUnifiedTypeSignature": "https://biomejs.dev/linter/rules/use-unified-type-signature",
    "lint/nursery/useUniqueElementIds": "https://biomejs.dev/linter/rules/use-unique-element-ids",
    "lint/performance/noAccumulatingSpread": "https://biomejs.dev/linter/rules/no-accumulating-spread",
    "lint/performance/noBarrelFile": "https://biomejs.dev/linter/rules/no-barrel-file",
    "lint/performance/noDelete": "https://biomejs.dev/linter/rules/no-delete",
    "lint/performance/noDynamicNamespaceImportAccess": "https://biomejs.dev/linter/rules/no-dynamic-namespace-import-access",
    "lint/performance/noImgElement": "https://biomejs.dev/linter/rules/no-img-element",
    "lint/performance/noNamespaceImport": "https://biomejs.dev/linter/rules/no-namespace-import",
    "lint/performance/noReExportAll": "https://biomejs.dev/linter/rules/no-re-export-all",
    "lint/performance/useTopLevelRegex": "https://biomejs.dev/linter/rules/use-top-level-regex",
    "lint/security/noBlankTarget": "https://biomejs.dev/linter/rules/no-blank-target",
    "lint/security/noDangerouslySetInnerHtml": "https://biomejs.dev/linter/rules/no-dangerously-set-inner-html",
    "lint/security/noDangerouslySetInnerHtmlWithChildren": "https://biomejs.dev/linter/rules/no-dangerously-set-inner-html-with-children",
    "lint/security/noGlobalEval": "https://biomejs.dev/linter/rules/no-global-eval",
    "lint/style/noCommonJs": "https://biomejs.dev/linter/rules/no-common-js",
    "lint/style/noDefaultExport": "https://biomejs.dev/linter/rules/no-default-export",
    "lint/style/noDescendingSpecificity": "https://biomejs.dev/linter/rules/no-descending-specificity",
    "lint/style/noDoneCallback": "https://biomejs.dev/linter/rules/no-done-callback",
    "lint/style/noEnum": "https://biomejs.dev/linter/rules/no-enum",
    "lint/style/noExportedImports": "https://biomejs.dev/linter/rules/no-exported-imports",
    "lint/style/noHeadElement": "https://biomejs.dev/linter/rules/no-head-element",
    "lint/style/noImplicitBoolean": "https://biomejs.dev/linter/rules/no-implicit-boolean",
    "lint/style/noInferrableTypes": "https://biomejs.dev/linter/rules/no-inferrable-types",
    "lint/style/noNamespace": "https://biomejs.dev/linter/rules/no-namespace",
    "lint/style/noNegationElse": "https://biomejs.dev/linter/rules/no-negation-else",
    "lint/style/noNestedTernary": "https://biomejs.dev/linter/rules/no-nested-ternary",
    "lint/style/noNonNullAssertion": "https://biomejs.dev/linter/rules/no-non-null-assertion",
    "lint/style/noParameterAssign": "https://biomejs.dev/linter/rules/no-parameter-assign",
    "lint/style/noParameterProperties": "https://biomejs.dev/linter/rules/no-parameter-properties",
    "lint/style/noProcessEnv": "https://biomejs.dev/linter/rules/no-process-env",
    "lint/style/noRestrictedGlobals": "https://biomejs.dev/linter/rules/no-restricted-globals",
    "lint/style/noRestrictedImports": "https://biomejs.dev/linter/rules/no-restricted-imports",
    "lint/style/noRestrictedTypes": "https://biomejs.dev/linter/rules/no-restricted-types",
    "lint/style/noShoutyConstants": "https://biomejs.dev/linter/rules/no-shouty-constants",
    "lint/style/noSubstr": "https://biomejs.dev/linter/rules/no-substr",
    "lint/style/noUnusedTemplateLiteral": "https://biomejs.dev/linter/rules/no-unused-template-literal",
    "lint/style/noUselessElse": "https://biomejs.dev/linter/rules/no-useless-else",
    "lint/style/noValueAtRule": "https://biomejs.dev/linter/rules/no-value-at-rule",
    "lint/style/noYodaExpression": "https://biomejs.dev/linter/rules/no-yoda-expression",
    "lint/style/useArrayLiterals": "https://biomejs.dev/linter/rules/use-array-literals",
    "lint/style/useAsConstAssertion": "https://biomejs.dev/linter/rules/use-as-const-assertion",
    "lint/style/useAtIndex": "https://biomejs.dev/linter/rules/use-at-index",
    "lint/style/useBlockStatements": "https://biomejs.dev/linter/rules/use-block-statements",
    "lint/style/useCollapsedElseIf": "https://biomejs.dev/linter/rules/use-collapsed-else-if",
    "lint/style/useCollapsedIf": "https://biomejs.dev/linter/rules/use-collapsed-if",
    "lint/style/useComponentExportOnlyModules": "https://biomejs.dev/linter/rules/use-component-export-only-modules",
    "lint/style/useConsistentArrayType": "https://biomejs.dev/linter/rules/use-consistent-array-type",
    "lint/style/useConsistentBuiltinInstantiation": "https://biomejs.dev/linter/rules/use-consistent-new-builtin",
    "lint/style/useConsistentCurlyBraces": "https://biomejs.dev/linter/rules/use-consistent-curly-braces",
    "lint/style/useConsistentMemberAccessibility": "https://biomejs.dev/linter/rules/use-consistent-member-accessibility",
    "lint/style/useConst": "https://biomejs.dev/linter/rules/use-const",
    "lint/style/useDefaultParameterLast": "https://biomejs.dev/linter/rules/use-default-parameter-last",
    "lint/style/useDefaultSwitchClause": "https://biomejs.dev/linter/rules/use-default-switch-clause",
    "lint/style/useDeprecatedReason": "https://biomejs.dev/linter/rules/use-deprecated-reason",
    "lint/style/useEnumInitializers": "https://biomejs.dev/linter/rules/use-enum-initializers",
    "lint/style/useExplicitLengthCheck": "https://biomejs.dev/linter/rules/use-explicit-length-check",
    "lint/style/useExponentiationOperator": "https://biomejs.dev/linter/rules/use-exponentiation-operator",
    "lint/style/useExportType": "https://biomejs.dev/linter/rules/use-export-type",
    "lint/style/useFilenamingConvention": "https://biomejs.dev/linter/rules/use-filenaming-convention",
    "lint/style/useForOf": "https://biomejs.dev/linter/rules/use-for-of",
    "lint/style/useFragmentSyntax": "https://biomejs.dev/linter/rules/use-fragment-syntax",
    "lint/style/useImportType": "https://biomejs.dev/linter/rules/use-import-type",
    "lint/style/useLiteralEnumMembers": "https://biomejs.dev/linter/rules/use-literal-enum-members",
    "lint/style/useNamingConvention": "https://biomejs.dev/linter/rules/use-naming-convention",
    "lint/style/useNodeAssertStrict": "https://biomejs.dev/linter/rules/use-node-assert-strict",
    "lint/style/useNodejsImportProtocol": "https://biomejs.dev/linter/rules/use-nodejs-import-protocol",
    "lint/style/useNumberNamespace": "https://biomejs.dev/linter/rules/use-number-namespace",
    "lint/style/useSelfClosingElements": "https://biomejs.dev/linter/rules/use-self-closing-elements",
    "lint/style/useShorthandArrayType": "https://biomejs.dev/linter/rules/use-shorthand-array-type",
    "lint/style/useShorthandAssign": "https://biomejs.dev/linter/rules/use-shorthand-assign",
    "lint/style/useShorthandFunctionType": "https://biomejs.dev/linter/rules/use-shorthand-function-type",
    "lint/style/useSingleCaseStatement": "https://biomejs.dev/linter/rules/use-single-case-statement",
    "lint/style/useSingleVarDeclarator": "https://biomejs.dev/linter/rules/use-single-var-declarator",
    "lint/style/useTemplate": "https://biomejs.dev/linter/rules/use-template",
    "lint/style/useThrowNewError": "https://biomejs.dev/linter/rules/use-throw-new-error",
    "lint/style/useThrowOnlyError": "https://biomejs.dev/linter/rules/use-throw-only-error",
    "lint/style/useTrimStartEnd": "https://biomejs.dev/linter/rules/use-trim-start-end",
    "lint/suspicious/noAlert": "https://biomejs.dev/linter/rules/no-alert",
    "lint/suspicious/noApproximativeNumericConstant": "https://biomejs.dev/linter/rules/no-approximative-numeric-constant",
    "lint/suspicious/noArrayIndexKey": "https://biomejs.dev/linter/rules/no-array-index-key",
    "lint/suspicious/noAssignInExpressions": "https://biomejs.dev/linter/rules/no-assign-in-expressions",
    "lint/suspicious/noAsyncPromiseExecutor": "https://biomejs.dev/linter/rules/no-async-promise-executor",
    "lint/suspicious/noCatchAssign": "https://biomejs.dev/linter/rules/no-catch-assign",
    "lint/suspicious/noClassAssign": "https://biomejs.dev/linter/rules/no-class-assign",
    "lint/suspicious/noCommentText": "https://biomejs.dev/linter/rules/no-comment-text",
    "lint/suspicious/noCompareNegZero": "https://biomejs.dev/linter/rules/no-compare-neg-zero",
    "lint/suspicious/noConfusingLabels": "https://biomejs.dev/linter/rules/no-confusing-labels",
    "lint/suspicious/noConfusingVoidType": "https://biomejs.dev/linter/rules/no-confusing-void-type",
    "lint/suspicious/noConsole": "https://biomejs.dev/linter/rules/no-console",
    "lint/suspicious/noConstEnum": "https://biomejs.dev/linter/rules/no-const-enum",
    "lint/suspicious/noControlCharactersInRegex": "https://biomejs.dev/linter/rules/no-control-characters-in-regex",
    "lint/suspicious/noDebugger": "https://biomejs.dev/linter/rules/no-debugger",
    "lint/suspicious/noDocumentCookie": "https://biomejs.dev/linter/rules/no-document-cookie",
    "lint/suspicious/noDocumentImportInPage": "https://biomejs.dev/linter/rules/no-document-import-in-page",
    "lint/suspicious/noDoubleEquals": "https://biomejs.dev/linter/rules/no-double-equals",
    "lint/suspicious/noDuplicateAtImportRules": "https://biomejs.dev/linter/rules/no-duplicate-at-import-rules",
    "lint/suspicious/noDuplicateCase": "https://biomejs.dev/linter/rules/no-duplicate-case",
    "lint/suspicious/noDuplicateClassMembers": "https://biomejs.dev/linter/rules/no-duplicate-class-members",
    "lint/suspicious/noDuplicateCustomProperties": "https://biomejs.dev/linter/rules/no-duplicate-custom-properties",
    "lint/suspicious/noDuplicateElseIf": "https://biomejs.dev/linter/rules/no-duplicate-else-if",
    "lint/suspicious/noDuplicateFields": "https://biomejs.dev/linter/rules/no-duplicate-fields",
    "lint/suspicious/noDuplicateFontNames": "https://biomejs.dev/linter/rules/no-font-family-duplicate-names",
    "lint/suspicious/noDuplicateJsxProps": "https://biomejs.dev/linter/rules/no-duplicate-jsx-props",
    "lint/suspicious/noDuplicateObjectKeys": "https://biomejs.dev/linter/rules/no-duplicate-object-keys",
    "lint/suspicious/noDuplicateParameters": "https://biomejs.dev/linter/rules/no-duplicate-parameters",
    "lint/suspicious/noDuplicateProperties": "https://biomejs.dev/linter/rules/no-duplicate-properties",
    "lint/suspicious/noDuplicateSelectorsKeyframeBlock": "https://biomejs.dev/linter/rules/no-duplicate-selectors-keyframe-block",
    "lint/suspicious/noDuplicateTestHooks": "https://biomejs.dev/linter/rules/no-duplicate-test-hooks",
    "lint/suspicious/noEmptyBlock": "https://biomejs.dev/linter/rules/no-empty-block",
    "lint/suspicious/noEmptyBlockStatements": "https://biomejs.dev/linter/rules/no-empty-block-statements",
    "lint/suspicious/noEmptyInterface": "https://biomejs.dev/linter/rules/no-empty-interface",
    "lint/suspicious/noEvolvingTypes": "https://biomejs.dev/linter/rules/no-evolving-types",
    "lint/suspicious/noExplicitAny": "https://biomejs.dev/linter/rules/no-explicit-any",
    "lint/suspicious/noExportsInTest": "https://biomejs.dev/linter/rules/no-exports-in-test",
    "lint/suspicious/noExtraNonNullAssertion": "https://biomejs.dev/linter/rules/no-extra-non-null-assertion",
    "lint/suspicious/noFallthroughSwitchClause": "https://biomejs.dev/linter/rules/no-fallthrough-switch-clause",
    "lint/suspicious/noFocusedTests": "https://biomejs.dev/linter/rules/no-focused-tests",
    "lint/suspicious/noFunctionAssign": "https://biomejs.dev/linter/rules/no-function-assign",
    "lint/suspicious/noGlobalAssign": "https://biomejs.dev/linter/rules/no-global-assign",
    "lint/suspicious/noGlobalIsFinite": "https://biomejs.dev/linter/rules/no-global-is-finite",
    "lint/suspicious/noGlobalIsNan": "https://biomejs.dev/linter/rules/no-global-is-nan",
    "lint/suspicious/noHeadImportInDocument": "https://biomejs.dev/linter/rules/no-head-import-in-document",
    "lint/suspicious/noImplicitAnyLet": "https://biomejs.dev/linter/rules/no-implicit-any-let",
    "lint/suspicious/noImportAssign": "https://biomejs.dev/linter/rules/no-import-assign",
    "lint/suspicious/noImportantInKeyframe": "https://biomejs.dev/linter/rules/no-important-in-keyframe",
    "lint/suspicious/noIrregularWhitespace": "https://biomejs.dev/linter/rules/no-irregular-whitespace",
    "lint/suspicious/noLabelVar": "https://biomejs.dev/linter/rules/no-label-var",
    "lint/suspicious/noMisleadingCharacterClass": "https://biomejs.dev/linter/rules/no-misleading-character-class",
    "lint/suspicious/noMisleadingInstantiator": "https://biomejs.dev/linter/rules/no-misleading-instantiator",
    "lint/suspicious/noMisplacedAssertion": "https://biomejs.dev/linter/rules/no-misplaced-assertion",
    "lint/suspicious/noMisrefactoredShorthandAssign": "https://biomejs.dev/linter/rules/no-misrefactored-shorthand-assign",
    "lint/suspicious/noOctalEscape": "https://biomejs.dev/linter/rules/no-octal-escape",
    "lint/suspicious/noPrototypeBuiltins": "https://biomejs.dev/linter/rules/no-prototype-builtins",
    "lint/suspicious/noReactSpecificProps": "https://biomejs.dev/linter/rules/no-react-specific-props",
    "lint/suspicious/noRedeclare": "https://biomejs.dev/linter/rules/no-redeclare",
    "lint/suspicious/noRedundantUseStrict": "https://biomejs.dev/linter/rules/no-redundant-use-strict",
    "lint/suspicious/noSelfCompare": "https://biomejs.dev/linter/rules/no-self-compare",
    "lint/suspicious/noShadowRestrictedNames": "https://biomejs.dev/linter/rules/no-shadow-restricted-names",
    "lint/suspicious/noShorthandPropertyOverrides": "https://biomejs.dev/linter/rules/no-shorthand-property-overrides",
    "lint/suspicious/noSkippedTests": "https://biomejs.dev/linter/rules/no-skipped-tests",
    "lint/suspicious/noSparseArray": "https://biomejs.dev/linter/rules/no-sparse-array",
    "lint/suspicious/noSuspiciousSemicolonInJsx": "https://biomejs.dev/linter/rules/no-suspicious-semicolon-in-jsx",
    "lint/suspicious/noTemplateCurlyInString": "https://biomejs.dev/linter/rules/no-template-curly-in-string",
    "lint/suspicious/noThenProperty": "https://biomejs.dev/linter/rules/no-then-property",
    "lint/suspicious/noUnsafeDeclarationMerging": "https://biomejs.dev/linter/rules/no-unsafe-declaration-merging",
    "lint/suspicious/noUnsafeNegation": "https://biomejs.dev/linter/rules/no-unsafe-negation",
    "lint/suspicious/noVar": "https://biomejs.dev/linter/rules/no-var",
    "lint/suspicious/noWith": "https://biomejs.dev/linter/rules/no-with",
    "lint/suspicious/useAdjacentOverloadSignatures": "https://biomejs.dev/linter/rules/use-adjacent-overload-signatures",
    "lint/suspicious/useAwait": "https://biomejs.dev/linter/rules/use-await",
    "lint/suspicious/useDefaultSwitchClauseLast": "https://biomejs.dev/linter/rules/use-default-switch-clause-last",
    "lint/suspicious/useErrorMessage": "https://biomejs.dev/linter/rules/use-error-message",
    "lint/suspicious/useGetterReturn": "https://biomejs.dev/linter/rules/use-getter-return",
    "lint/suspicious/useGoogleFontDisplay": "https://biomejs.dev/linter/rules/use-google-font-display",
    "lint/suspicious/useGuardForIn": "https://biomejs.dev/linter/rules/use-guard-for-in",
    "lint/suspicious/useIsArray": "https://biomejs.dev/linter/rules/use-is-array",
    "lint/suspicious/useNamespaceKeyword": "https://biomejs.dev/linter/rules/use-namespace-keyword",
    "lint/suspicious/useNumberToFixedDigitsArgument": "https://biomejs.dev/linter/rules/use-number-to-fixed-digits-argument",
    "lint/suspicious/useStrictMode": "https://biomejs.dev/linter/rules/use-strict-mode",
    // end lint rules
    // start assist actions
    "assist/source/useSortedKeys": "https://biomejs.dev/assist/actions/use-sorted-keys",
    "assist/source/useSortedProperties": "https://biomejs.dev/assist/actions/use-sorted-properties",
    "assist/source/useSortedAttributes": "https://biomejs.dev/assist/actions/use-sorted-attributes",
    "assist/source/organizeImports": "https://biomejs.dev/assist/actions/organize-imports",
    // end assist actions
    ; // start syntax rules
    "syntax/correctness/noTypeOnlyImportAttributes",
    "syntax/correctness/noSuperWithoutExtends",
    "syntax/correctness/noInitializerWithDefinite",
    "syntax/correctness/noDuplicatePrivateClassMembers",
    // end syntax rules

    // General categories
    "files/missingHandler",
    "format",
    "check",
    "ci",
    "stdin",
    "configuration",
    "assist",
    "migrate",
    "deserialize",
    "plugin",
    "project",
    "search",
    "internalError/io",
    "internalError/fs",
    "internalError/panic",
    "reporter/parse",
    "reporter/format",
    "reporter/violations",
    // parse categories
    "parse",

    // Lint groups
    "lint",
    "lint/a11y",
    "lint/complexity",
    "lint/correctness",
    "lint/nursery",
    "lint/performance",
    "lint/security",
    "lint/style",
    "lint/suspicious",
    "lint/plugin",

    // Suppression comments
    "suppressions/parse",
    "suppressions/unknownGroup",
    "suppressions/unknownRule",
    "suppressions/unknownAction",
    "suppressions/unused",
    "suppressions/incorrect",
    // Used in tests and examples
    "args/fileNotFound",
    "flags/invalid",
    "semanticTests",
}
