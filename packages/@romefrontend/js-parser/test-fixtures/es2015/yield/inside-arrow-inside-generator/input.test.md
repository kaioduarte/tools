# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romefrontend/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > yield > inside-arrow-inside-generator`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "input.js"
		end: Object {
			column: 1
			index: 52
			line: 4
		}
		start: Object {
			column: 0
			index: 0
			line: 1
		}
	}
	body: Array [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "fn"
				loc: Object {
					filename: "input.js"
					identifierName: "fn"
					end: Object {
						column: 12
						index: 12
						line: 1
					}
					start: Object {
						column: 10
						index: 10
						line: 1
					}
				}
			}
			loc: Object {
				filename: "input.js"
				end: Object {
					column: 1
					index: 52
					line: 4
				}
				start: Object {
					column: 0
					index: 0
					line: 1
				}
			}
			head: JSFunctionHead {
				async: false
				generator: true
				hasHoistedVars: false
				params: Array []
				rest: undefined
				returnType: undefined
				thisType: undefined
				typeParameters: undefined
				loc: Object {
					filename: "input.js"
					end: Object {
						column: 14
						index: 14
						line: 1
					}
					start: Object {
						column: 12
						index: 12
						line: 1
					}
				}
			}
			body: JSBlockStatement {
				directives: Array []
				loc: Object {
					filename: "input.js"
					end: Object {
						column: 1
						index: 52
						line: 4
					}
					start: Object {
						column: 15
						index: 15
						line: 1
					}
				}
				body: Array [
					JSExpressionStatement {
						loc: Object {
							filename: "input.js"
							end: Object {
								column: 14
								index: 31
								line: 2
							}
							start: Object {
								column: 2
								index: 19
								line: 2
							}
						}
						expression: JSArrowFunctionExpression {
							loc: Object {
								filename: "input.js"
								end: Object {
									column: 13
									index: 30
									line: 2
								}
								start: Object {
									column: 2
									index: 19
									line: 2
								}
							}
							body: JSReferenceIdentifier {
								name: "yield"
								loc: Object {
									filename: "input.js"
									identifierName: "yield"
									end: Object {
										column: 13
										index: 30
										line: 2
									}
									start: Object {
										column: 8
										index: 25
										line: 2
									}
								}
							}
							head: JSFunctionHead {
								async: false
								hasHoistedVars: false
								params: Array []
								rest: undefined
								returnType: undefined
								thisType: undefined
								loc: Object {
									filename: "input.js"
									end: Object {
										column: 7
										index: 24
										line: 2
									}
									start: Object {
										column: 2
										index: 19
										line: 2
									}
								}
							}
						}
					}
					JSExpressionStatement {
						loc: Object {
							filename: "input.js"
							end: Object {
								column: 18
								index: 50
								line: 3
							}
							start: Object {
								column: 2
								index: 34
								line: 3
							}
						}
						expression: JSArrowFunctionExpression {
							loc: Object {
								filename: "input.js"
								end: Object {
									column: 17
									index: 49
									line: 3
								}
								start: Object {
									column: 2
									index: 34
									line: 3
								}
							}
							head: JSFunctionHead {
								async: false
								hasHoistedVars: false
								params: Array []
								rest: undefined
								returnType: undefined
								thisType: undefined
								loc: Object {
									filename: "input.js"
									end: Object {
										column: 7
										index: 39
										line: 3
									}
									start: Object {
										column: 2
										index: 34
										line: 3
									}
								}
							}
							body: JSBlockStatement {
								directives: Array []
								loc: Object {
									filename: "input.js"
									end: Object {
										column: 17
										index: 49
										line: 3
									}
									start: Object {
										column: 8
										index: 40
										line: 3
									}
								}
								body: Array [
									JSExpressionStatement {
										loc: Object {
											filename: "input.js"
											end: Object {
												column: 15
												index: 47
												line: 3
											}
											start: Object {
												column: 10
												index: 42
												line: 3
											}
										}
										expression: JSReferenceIdentifier {
											name: "yield"
											loc: Object {
												filename: "input.js"
												identifierName: "yield"
												end: Object {
													column: 15
													index: 47
													line: 3
												}
												start: Object {
													column: 10
													index: 42
													line: 3
												}
											}
										}
									}
								]
							}
						}
					}
				]
			}
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```