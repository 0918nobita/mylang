var searchIndex = JSON.parse('{\
"mylang_ast":{"doc":"抽象構文木の定義","t":[13,4,13,13,13,4,13,0,0,13,4,13,13,11,11,11,11,11,11,11,11,11,11,11,13,13,4,11,11,11,11,11,11,11,11,11,11,11],"n":["Add","Expr","I32Lit","PrintI32","PrintStr","Stmt","StrLit","expr","stmt","Add","Expr","I32Lit","StrLit","borrow","borrow_mut","deserialize","fmt","from","into","locate","serialize","try_from","try_into","type_id","PrintI32","PrintStr","Stmt","borrow","borrow_mut","deserialize","fmt","from","into","locate","serialize","try_from","try_into","type_id"],"q":["mylang_ast","","","","","","","","","mylang_ast::expr","","","","","","","","","","","","","","","mylang_ast::stmt","","","","","","","","","","","","",""],"d":["","式を表す抽象構文木","","","","文を表す抽象構文木","","","","","式を表す抽象構文木","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","文を表す抽象構文木","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","",""],"i":[1,0,1,7,7,0,1,0,0,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,7,7,0,7,7,7,7,7,7,7,7,7,7,7],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[],[[2,[1]]]],[[1,3],4],[[]],[[]],[1,5],[1,2],[[],2],[[],2],[[],6],0,0,0,[[]],[[]],[[],[[2,[7]]]],[[7,3],4],[[]],[[]],[7,5],[7,2],[[],2],[[],2],[[],6]],"p":[[4,"Expr"],[4,"Result"],[3,"Formatter"],[6,"Result"],[3,"Range"],[3,"TypeId"],[4,"Stmt"]]},\
"mylang_ast_interp":{"doc":"抽象構文木インタプリタ","t":[4,6,13,11,11,0,5,5,11,11,11,11,11,11,11,11,11,12,12,4,13,13,3,4,13,13,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["AstInterpError","AstInterpResult","TypeMismatch","borrow","borrow_mut","entity","eval_expr","execute","fmt","fmt","from","into","provide","to_string","try_from","try_into","type_id","actual","expected","Entity","I32","I32","I32Entity","RuntimeTypeInfo","Str","Str","StrEntity","add","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","get_type","into","into","into","into","new","new","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id"],"q":["mylang_ast_interp","","","","","","","","","","","","","","","","","mylang_ast_interp::AstInterpError","","mylang_ast_interp::entity","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","…","","抽象構文木を解釈実行する","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","バイトコード実行時に扱う値","","","符号付き32ビット整数値","実行時型情報","","","文字列データ","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","",""],"i":[0,0,4,4,4,0,0,0,4,4,4,4,4,4,4,4,4,15,15,0,13,2,0,0,13,2,0,11,11,12,13,2,11,12,13,2,11,11,12,12,13,13,2,11,12,13,2,2,11,12,13,2,11,12,11,12,13,11,12,13,2,11,12,13,2,11,12,13,2],"f":[0,0,0,[[]],[[]],0,[1,[[3,[2]]]],[[],3],[[4,5],6],[[4,5],6],[[]],[[]],[7],[[],8],[[],9],[[],9],[[],10],0,0,0,0,0,0,0,0,0,0,[[11,11],11],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[11,5],6],[[11,5],6],[[12,5],6],[[12,5],6],[[13,5],6],[[13,5],6],[[2,5],6],[[]],[[]],[[]],[[]],[2,13],[[]],[[]],[[]],[[]],[14,11],[8,12],[[],8],[[],8],[[],8],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],10],[[],10],[[],10],[[],10]],"p":[[4,"Expr"],[4,"Entity"],[6,"AstInterpResult"],[4,"AstInterpError"],[3,"Formatter"],[6,"Result"],[3,"Demand"],[3,"String"],[4,"Result"],[3,"TypeId"],[3,"I32Entity"],[3,"StrEntity"],[4,"RuntimeTypeInfo"],[15,"i32"],[13,"TypeMismatch"]]},\
"mylang_bytecode":{"doc":"バイトコードの定義","t":[13,13,13,4,13,13,13,13,11,11,11,11,11,11,11,11,11,11],"n":["Call","I32Add","I32Const","Inst","PrintI32","PrintStr","Return","StrConst","borrow","borrow_mut","deserialize","fmt","from","into","serialize","try_from","try_into","type_id"],"q":["mylang_bytecode","","","","","","","","","","","","","","","","",""],"d":["PC + 1 をスタックに push …","","","命令","","","スタックトから pop した番地にジャンプする","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","",""],"i":[1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[0,0,0,0,0,0,0,0,[[]],[[]],[[],[[2,[1]]]],[[1,3],4],[[]],[[]],[1,2],[[],2],[[],2],[[],5]],"p":[[4,"Inst"],[4,"Result"],[3,"Formatter"],[6,"Result"],[3,"TypeId"]]},\
"mylang_bytecode_compiler":{"doc":"抽象構文木→バイトコード変換器","t":[5,5,5],"n":["ast_to_bytecode","expr_to_bytecode","stmt_to_bytecode"],"q":["mylang_bytecode_compiler","",""],"d":["","",""],"i":[0,0,0],"f":[[[],[[2,[1]]]],[3,[[2,[1]]]],[4,[[2,[1]]]]],"p":[[4,"Inst"],[3,"Vec"],[4,"Expr"],[4,"Stmt"]]},\
"mylang_cli_ext":{"doc":"","t":[13,3,8,7,4,13,8,8,11,11,11,11,12,0,11,11,11,0,10,12,5,5,10,11,11,11,11,11,11,11,0,10,5,5,3,8,11,11,12,11,11,11,12,10,11,11,11,11,8,10,3,3,8,11,11,11,11,12,11,11,11,11,11,12,11,11,11,12,12,11,11,11,11,11,11,10],"n":["Binary","CommandFromParser","CommandFromParserExt","FILE_FORMAT_PARSER","FileFormat","Json","MatchesFromParser","WithOutputExt","borrow","borrow_mut","clone","clone_into","cmd","command","from","from_str","into","matches","parse","parser","read","reader_from_stdin_or_file","to_command","to_owned","to_possible_value","try_from","try_into","type_id","value_of","value_variants","with_output","with_output","write","writer_to_stdout_or_file","CommandFromParser","CommandFromParserExt","borrow","borrow_mut","cmd","from","into","new","parser","to_command","try_from","try_into","type_id","with_output","MatchesFromParser","parse","MatchesWithOutput","ParserWithOutput","WithOutputExt","borrow","borrow","borrow_mut","borrow_mut","cmd","from","from","get_matches","into","into","matches","new","output","parse","parser","parser","try_from","try_from","try_into","try_into","type_id","type_id","with_output"],"q":["mylang_cli_ext","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","mylang_cli_ext::command","","","","","","","","","","","","","","mylang_cli_ext::matches","","mylang_cli_ext::with_output","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","",""],"i":[1,0,0,0,0,1,0,0,1,1,1,1,11,0,1,1,1,0,21,11,0,0,22,1,1,1,1,1,1,1,0,23,0,0,0,0,11,11,11,11,11,11,11,22,11,11,11,11,0,21,0,0,0,20,16,20,16,16,20,16,16,20,16,20,16,20,20,20,16,20,16,20,16,20,16,23],"f":[0,0,0,0,0,0,0,0,[[]],[[]],[1,1],[[]],0,0,[[]],[2,[[3,[1]]]],[[]],0,[[]],0,[[[5,[4]],1],6],[[7,8,[10,[9]]],[[6,[[5,[4]]]]]],[[],11],[[]],[1,[[10,[12]]]],[[],3],[[],3],[[],13],[[14,2],[[3,[1,15]]]],[[]],0,[[],16],[[[5,[17]],1],6],[[18,8,[10,[9]]],[[6,[[5,[17]]]]]],0,0,[[]],[[]],0,[[]],[[]],[19,11],0,[[],11],[[],3],[[],3],[[],13],[11,16],0,[[]],0,0,0,[[]],[[]],[[]],[[]],0,[[]],[[]],[16,20],[[]],[[]],0,[11,16],[[20,18],[[6,[[5,[17]]]]]],[20],0,0,[[],3],[[],3],[[],3],[[],3],[[],13],[[],13],[[],16]],"p":[[4,"FileFormat"],[15,"str"],[4,"Result"],[8,"BufRead"],[3,"Box"],[6,"Result"],[3,"Stdin"],[15,"bool"],[3,"String"],[4,"Option"],[3,"CommandFromParser"],[3,"PossibleValue"],[3,"TypeId"],[3,"ArgMatches"],[3,"Error"],[3,"ParserWithOutput"],[8,"Write"],[3,"Stdout"],[3,"Command"],[3,"MatchesWithOutput"],[8,"MatchesFromParser"],[8,"CommandFromParserExt"],[8,"WithOutputExt"]]},\
"mylang_cli_test":{"doc":"","t":[3,11,11,11,11,11,11,11,11,11,11,12,11,5,11,11,11,11,11,11],"n":["Cli","augment_args","augment_args_for_update","borrow","borrow_mut","command","command_for_update","from","from_arg_matches","from_arg_matches_mut","group_id","input","into","main","to_command","try_from","try_into","type_id","update_from_arg_matches","update_from_arg_matches_mut"],"q":["mylang_cli_test","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","Returns the argument unchanged.","","","","","Calls <code>U::from(self)</code>.","","","","","","",""],"i":[0,3,3,3,3,3,3,3,3,3,3,3,3,0,3,3,3,3,3,3],"f":[0,[1,1],[1,1],[[]],[[]],[[],1],[[],1],[[]],[2,[[5,[3,4]]]],[2,[[5,[3,4]]]],[[],[[7,[6]]]],0,[[]],[[],8],[[],9],[[],5],[[],5],[[],10],[[3,2],[[5,[4]]]],[[3,2],[[5,[4]]]]],"p":[[3,"Command"],[3,"ArgMatches"],[3,"Cli"],[6,"Error"],[4,"Result"],[3,"Id"],[4,"Option"],[6,"Result"],[3,"CommandFromParser"],[3,"TypeId"]]},\
"mylang_lexer":{"doc":"字句解析器","t":[13,13,13,3,4,8,13,11,11,11,11,11,12,5,10,11,0,0,12,0,11,11,11,0,13,13,13,4,6,13,11,11,11,11,11,11,11,11,11,11,11,11,13,13,4,13,13,11,11,11,11,11,11,0,11,0,0,11,11,11,11,3,12,11,11,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,3,12,11,11,11,11,11,12,12,11,11,11,11,12,11,11,11,11,11,11,3,12,11,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,0,0,0,0,5,13,4,13,11,11,11,5,11,11,11,11,5,13,13,13,4,11,11,11,11,5,11,11,11,13,13,4,11,11,11,11,5,11,11,11,3,8,11,11,12,11,11,11,11,11,12,11,11,11,10],"n":["ForbiddenChar","InvalidEscapeSequence","InvalidKeyword","Lex","LexErr","LexExt","MissingClosingQuoteForStr","borrow","borrow_mut","from","into","into_iter","iter","lex","lex","next","result","state","state","transition","try_from","try_into","type_id","with_pos","ForbiddenChar","InvalidEscapeSequence","InvalidKeyword","LexErr","LexResult","MissingClosingQuoteForStr","borrow","borrow_mut","fmt","fmt","from","into","locate","provide","to_string","try_from","try_into","type_id","I32","Initial","State","Str","Symbol","borrow","borrow_mut","clone","clone_into","fmt","from","i32","into","str","symbol","to_owned","try_from","try_into","type_id","I32State","acc","append_digit_char","borrow","borrow_mut","clone","clone_into","default","end","fmt","from","into","new","start","to_owned","tokenize","try_from","try_into","type_id","StrState","acc","borrow","borrow_mut","clone","clone_into","default","end","escape","fmt","from","into","new","start","to_owned","tokenize","try_append_char","try_from","try_into","type_id","SymbolState","acc","append_char","borrow","borrow_mut","clone","clone_into","end","fmt","from","into","new","start","to_owned","tokenize","try_from","try_into","type_id","i32","initial","str","symbol","transition","Continued","I32LexResult","Interrupted","borrow","borrow_mut","from","i32_lex","into","try_from","try_into","type_id","initial_lex","Completed","Continued","Err","StrLexResult","borrow","borrow_mut","from","into","str_lex","try_from","try_into","type_id","Continued","Interrupted","SymbolLexResult","borrow","borrow_mut","from","into","symbol_lex","try_from","try_into","type_id","WithPos","WithPosExt","borrow","borrow_mut","chars","from","into","into_iter","multiunzip","next","pos","try_from","try_into","type_id","with_pos"],"q":["mylang_lexer","","","","","","","","","","","","","","","","","","","","","","","","mylang_lexer::result","","","","","","","","","","","","","","","","","","mylang_lexer::state","","","","","","","","","","","","","","","","","","","mylang_lexer::state::i32","","","","","","","","","","","","","","","","","","","mylang_lexer::state::str","","","","","","","","","","","","","","","","","","","","mylang_lexer::state::symbol","","","","","","","","","","","","","","","","","","mylang_lexer::transition","","","","","mylang_lexer::transition::i32","","","","","","","","","","","mylang_lexer::transition::initial","mylang_lexer::transition::str","","","","","","","","","","","","mylang_lexer::transition::symbol","","","","","","","","","","","mylang_lexer::with_pos","","","","","","","","","","","","","",""],"d":["","","","…","字句解析中に発生するエラー","…","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","字句解析を実行する","…","…","字句解析の結果","字句解析器の状態","","字句解析器内部の状態遷移の実装","","","","…","","","","字句解析中に発生するエラー","字句解析の結果","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","初期状態","字句解析器内部の状態","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","…","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","",""],"i":[6,6,6,0,0,0,6,2,2,2,2,2,2,0,23,2,0,0,2,0,2,2,2,0,6,6,6,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,12,12,0,12,12,12,12,12,12,12,12,0,12,0,0,12,12,12,12,0,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,0,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,0,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,0,0,0,0,0,19,0,19,19,19,19,0,19,19,19,19,0,20,20,20,0,20,20,20,20,0,20,20,20,21,21,0,21,21,21,21,0,21,21,21,0,0,22,22,22,22,22,22,22,22,22,22,22,22,24],"f":[0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],0,[[],1],[[],2],[2,3],0,0,0,0,[[],4],[[],4],[[],5],0,0,0,0,0,0,0,[[]],[[]],[[6,7],8],[[6,7],8],[[]],[[]],[6,9],[10],[[],11],[[],4],[[],4],[[],5],0,0,0,0,0,[[]],[[]],[12,12],[[]],[[12,7],8],[[]],0,[[]],0,0,[[]],[[],4],[[],4],[[],5],0,0,[[13,14,15],13],[[]],[[]],[13,13],[[]],[[],13],0,[[13,7],8],[[]],[[]],[[14,11],13],0,[[]],[13,16],[[],4],[[],4],[[],5],0,0,[[]],[[]],[17,17],[[]],[[],17],0,0,[[17,7],8],[[]],[[]],[14,17],0,[[]],[17,16],[[17,14,15],[[4,[17,6]]]],[[],4],[[],4],[[],5],0,0,[[18,14,15],18],[[]],[[]],[18,18],[[]],0,[[18,7],8],[[]],[[]],[[14,11],18],0,[[]],[18,16],[[],4],[[],4],[[],5],0,0,0,0,[12],0,0,0,[[]],[[]],[[]],[13,19],[[]],[[],4],[[],4],[[],5],[[]],0,0,0,0,[[]],[[]],[[]],[[]],[17,20],[[],4],[[],4],[[],5],0,0,0,[[]],[[]],[[]],[[]],[18,21],[[],4],[[],4],[[],5],0,0,[[]],[[]],0,[[]],[[]],[[]],[[]],[22,3],0,[[],4],[[],4],[[],5],[[],22]],"p":[[8,"Iterator"],[3,"Lex"],[4,"Option"],[4,"Result"],[3,"TypeId"],[4,"LexErr"],[3,"Formatter"],[6,"Result"],[3,"Range"],[3,"Demand"],[3,"String"],[4,"State"],[3,"I32State"],[3,"Pos"],[15,"char"],[4,"Token"],[3,"StrState"],[3,"SymbolState"],[4,"I32LexResult"],[4,"StrLexResult"],[4,"SymbolLexResult"],[3,"WithPos"],[8,"LexExt"],[8,"WithPosExt"]]},\
"mylang_lsp_server":{"doc":"mylang を記述するための、LSP …","t":[5,0,0,0,0,4,13,13,13,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,3,11,11,11,11,11,12,11,11,11,11,3,0,11,11,0,12,11,11,0,11,11,0,12,11,11,11,5,5,5,5,5,5,5,5,5,5,5,5,3,11,11,11,11,11,11,11,11],"n":["launch_lsp_server","message","receiver","responder","sender","LspMessage","Notification","Request","Response","borrow","borrow_mut","deserialize","fmt","from","into","raw_message","serialize","try_from","try_into","type_id","id","id","method","method","params","params","result","Receiver","borrow","borrow_mut","from","into","new","responder","started","try_from","try_into","type_id","Responder","analyzer","borrow","borrow_mut","diagnostic","diagnostics_supported","from","handle","handler","into","new","range","sender","try_from","try_into","type_id","analyze_src","lex_all","lex_err_to_diagnostic","parse_err_to_diagnostic","handle","handle_did_change_notification","handle_did_close_notification","handle_did_open_notification","handle_initialize_request","text_document_uri","token_types","locatable_to_lsp_range","Sender","borrow","borrow_mut","from","handle","into","try_from","try_into","type_id"],"q":["mylang_lsp_server","","","","","mylang_lsp_server::message","","","","","","","","","","","","","","","mylang_lsp_server::message::LspMessage","","","","","","","mylang_lsp_server::receiver","","","","","","","","","","","mylang_lsp_server::responder","","","","","","","","","","","","","","","","mylang_lsp_server::responder::analyzer","","mylang_lsp_server::responder::diagnostic","","mylang_lsp_server::responder::handler","","","","","","","mylang_lsp_server::responder::range","mylang_lsp_server::sender","","","","","","","",""],"d":["…","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","…","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","…","","","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","…","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","",""],"i":[0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,22,23,22,24,22,24,23,0,9,9,9,9,9,9,9,9,9,9,0,0,7,7,0,7,7,7,0,7,7,0,7,7,7,7,0,0,0,0,0,0,0,0,0,0,0,0,0,10,10,10,10,10,10,10,10],"f":[[[]],0,0,0,0,0,0,0,0,[[]],[[]],[[],[[2,[1]]]],[[1,3],4],[[]],[[]],[1,5],[1,2],[[],2],[[],2],[[],6],0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[[8,[7]]],9],0,[9],[[],2],[[],2],[[],6],0,0,[[]],[[]],0,0,[[]],[[7,1]],0,[[]],[[[8,[10]]],7],0,0,[[],2],[[],2],[[],6],[[[8,[10]],11,12],13],[12],[14,11],[15,11],[[[8,[10]],[17,[16]],1]],[[[8,[10]],11]],[[[8,[10]],11]],[[[8,[10]],11]],[[[8,[10]],[17,[16]],18,11]],[11,[[19,[11]]]],[11,[[19,[[20,[5,18]]]]]],[[],11],0,[[]],[[]],[[]],[[10,1,21]],[[]],[[],2],[[],2],[[],6]],"p":[[4,"LspMessage"],[4,"Result"],[3,"Formatter"],[6,"Result"],[3,"String"],[3,"TypeId"],[3,"Responder"],[3,"Addr"],[3,"Receiver"],[3,"Sender"],[4,"Value"],[15,"str"],[6,"Result"],[4,"LexErr"],[4,"ParseErr"],[3,"AtomicBool"],[3,"Arc"],[15,"usize"],[4,"Option"],[3,"HashMap"],[3,"Context"],[13,"Request"],[13,"Response"],[13,"Notification"]]},\
"mylang_parser":{"doc":"構文解析器","t":[13,4,6,13,0,5,0,0,5,0,0,0,5,5,5,5,13,4,6,13,11,11,11,11,11,11,11,11,11,11,11,11],"n":["KeywordExpected","ParseErr","ParseResult","TermExpected","parse","parse","result","expr","parse","program","stmt","term","expr","program","stmt","term","KeywordExpected","ParseErr","ParseResult","TermExpected","borrow","borrow_mut","fmt","fmt","from","into","locate","provide","to_string","try_from","try_into","type_id"],"q":["mylang_parser","","","","","","","mylang_parser::parse","","","","","mylang_parser::parse::expr","mylang_parser::parse::program","mylang_parser::parse::stmt","mylang_parser::parse::term","mylang_parser::result","","","","","","","","","","","","","","",""],"d":["","構文解析中に発生するエラー","","","","","","","","","","","","","","","","構文解析中に発生するエラー","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","",""],"i":[7,0,0,7,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,7,7,7,7,7,7,7,7,7,7,7,7,7],"f":[0,0,0,0,0,[1,[[4,[[3,[2]]]]]],0,0,[1,[[4,[[3,[2]]]]]],0,0,0,[[5,6],3],[5,[[4,[[3,[2]]]]]],[[5,6],[[3,[2]]]],[[5,6],3],0,0,0,0,[[]],[[]],[[7,8],9],[[7,8],9],[[]],[[]],[7,10],[11],[[],12],[[],13],[[],13],[[],14]],"p":[[8,"Iterator"],[4,"Stmt"],[6,"ParseResult"],[3,"Vec"],[3,"PutBack"],[3,"Pos"],[4,"ParseErr"],[3,"Formatter"],[6,"Result"],[3,"Range"],[3,"Demand"],[3,"String"],[4,"Result"],[3,"TypeId"]]},\
"mylang_token":{"doc":"…","t":[13,13,13,13,13,4,13,8,13,3,13,13,3,13,4,11,11,11,11,12,11,11,12,11,11,11,11,11,11,11,12,0,10,11,0,14,0,14,11,11,12,11,11,11,11,11,11,8,10,3,11,11,12,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,3,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11],"n":["AddOp","Equal","I32","Ident","Keyword","KeywordKind","Let","Locatable","Newline","Pos","PrintI32","PrintStr","Range","Str","Token","borrow","borrow","borrow_mut","borrow_mut","character","deserialize","deserialize","end","fmt","fmt","from","from","from_str","into","into","line","locatable","locate","locate","pos","pos","range","range","serialize","serialize","start","try_from","try_from","try_into","try_into","type_id","type_id","Locatable","locate","Pos","borrow","borrow_mut","character","clone","clone_into","default","deserialize","eq","fmt","fmt","from","into","line","new","next_char","next_line","serialize","to_owned","to_string","try_from","try_into","type_id","Range","borrow","borrow_mut","clone","clone_into","concat","default","deserialize","end","end","end_ref","eq","fmt","fmt","from","from","into","new","serialize","start","start_ref","to_owned","to_string","try_from","try_into","type_id"],"q":["mylang_token","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","mylang_token::locatable","","mylang_token::pos","","","","","","","","","","","","","","","","","","","","","","","mylang_token::range","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","…","","ソースコード上の位置","","","ソースコード中の範囲","","","","","","","列番号","","","終端","","","Returns the argument unchanged.","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","行番号","","対応する範囲を返す","","","<code>Pos</code> を簡単に生成するためのマクロ","","<code>Range</code> を簡単に生成するためのマクロ","","","始端","","","","","","","…","対応する範囲を返す","ソースコード上の位置","","","列番号","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","行番号","新しい位置を生成して返す","次の文字位置に移動する …","次の行の先頭に移動する","","","","","","","ソースコード中の範囲","","","","","終端を別の範囲の終端に設定する","","","所有権をムーブして終端を返す","終端","終端の参照を返す","","","","始端と終端が同じ位置の範囲に変換する","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","新しい範囲を生成して返す","","始端","始端の参照を返す","","","","",""],"i":[3,3,3,3,3,0,1,0,3,0,1,1,0,3,0,1,3,1,3,9,1,3,7,1,3,1,3,1,1,3,9,0,13,3,0,0,0,0,1,3,7,1,3,1,3,1,3,0,13,0,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,0,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],0,[[],[[2,[1]]]],[[],[[2,[3]]]],0,[[1,4],5],[[3,4],5],[[]],[[]],[6,[[2,[1]]]],[[]],[[]],0,0,[[],7],[3,7],0,0,0,0,[1,2],[3,2],0,[[],2],[[],2],[[],2],[[],2],[[],8],[[],8],0,[[],7],0,[[]],[[]],0,[9,9],[[]],[[],9],[[],[[2,[9]]]],[[9,9],10],[[9,4],5],[[9,4],5],[[]],[[]],0,[[11,11],9],[9],[9],[9,2],[[]],[[],12],[[],2],[[],2],[[],8],0,[[]],[[]],[7,7],[[]],[[7,7]],[[],7],[[],[[2,[7]]]],[7,9],0,[7,9],[[7,7],10],[[7,4],5],[[7,4],5],[9,7],[[]],[[]],[[9,9],7],[7,2],0,[7,9],[[]],[[],12],[[],2],[[],2],[[],8]],"p":[[4,"KeywordKind"],[4,"Result"],[4,"Token"],[3,"Formatter"],[6,"Result"],[15,"str"],[3,"Range"],[3,"TypeId"],[3,"Pos"],[15,"bool"],[15,"u32"],[3,"String"],[8,"Locatable"]]},\
"mylang_vm":{"doc":"バイトコードインタプリタ","t":[13,13,4,6,11,11,5,0,5,11,11,11,5,5,11,5,5,11,5,5,11,11,11,11,12,12,13,13,4,13,13,3,4,13,13,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["StackUnderflow","TypeMismatch","VMError","VMResult","borrow","borrow_mut","call","entity","execute","fmt","fmt","from","i32_add","i32_const","into","print_i32","print_str","provide","ret","str_const","to_string","try_from","try_into","type_id","actual","expected","Addr","Addr","Entity","I32","I32","I32Entity","RuntimeTypeInfo","Str","Str","StrEntity","add","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","get_type","into","into","into","into","new","new","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id"],"q":["mylang_vm","","","","","","","","","","","","","","","","","","","","","","","","mylang_vm::VMError","","mylang_vm::entity","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["…","…","…","バイトコードの実行結果","","","","仮想マシンで扱う「実体」の定義","バイトコードを解釈実行する","","","Returns the argument unchanged.","I32Add 命令を実行する","I32Const 命令を実行する","Calls <code>U::from(self)</code>.","PrintI32 命令を実行する","PrintStr 命令を実行する","","","StrConst 命令を実行する","","","","","","","","","バイトコード実行時に扱う値","","","符号付き32ビット整数値","実行時型情報","","","文字列データ","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","",""],"i":[3,3,0,0,3,3,0,0,0,3,3,3,0,0,3,0,0,3,0,0,3,3,3,3,16,16,14,15,0,14,15,0,0,14,15,0,12,12,13,14,15,12,13,14,15,12,12,13,13,14,14,15,12,13,14,15,15,12,13,14,15,12,13,12,13,14,12,13,14,15,12,13,14,15,12,13,14,15],"f":[0,0,0,0,[[]],[[]],[[1,2,2],[[4,[3]]]],0,[[],[[4,[3]]]],[[3,5],6],[[3,5],6],[[]],[1,[[4,[3]]]],[[1,7]],[[]],[1,[[4,[3]]]],[1,[[4,[3]]]],[8],[[1,2],[[4,[3]]]],[[1,9]],[[],10],[[],4],[[],4],[[],11],0,0,0,0,0,0,0,0,0,0,0,0,[[12,12],12],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[12,5],6],[[12,5],6],[[13,5],6],[[13,5],6],[[14,5],6],[[14,5],6],[[15,5],6],[[]],[[]],[[]],[[]],[15,14],[[]],[[]],[[]],[[]],[7,12],[10,13],[[],10],[[],10],[[],10],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],11],[[],11],[[],11],[[],11]],"p":[[3,"Vec"],[15,"usize"],[4,"VMError"],[4,"Result"],[3,"Formatter"],[6,"Result"],[15,"i32"],[3,"Demand"],[15,"str"],[3,"String"],[3,"TypeId"],[3,"I32Entity"],[3,"StrEntity"],[4,"RuntimeTypeInfo"],[4,"Entity"],[13,"TypeMismatch"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
