var searchIndex = JSON.parse('{\
"mylang_ast":{"doc":"抽象構文木の定義","t":[13,4,13,13,13,4,13,0,0,12,12,12,12,12,12,12,12,12,12,13,4,13,13,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,13,13,4,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12],"n":["Add","Expr","I32Lit","PrintI32","PrintStr","Stmt","StrLit","expr","stmt","0","0","0","1","1","1","0","0","1","1","Add","Expr","I32Lit","StrLit","borrow","borrow_mut","deserialize","fmt","from","into","locate","serialize","try_from","try_into","type_id","0","0","0","1","1","1","PrintI32","PrintStr","Stmt","borrow","borrow_mut","deserialize","fmt","from","into","locate","serialize","try_from","try_into","type_id","0","0","1","1"],"q":["mylang_ast","","","","","","","","","mylang_ast::Expr","","","","","","mylang_ast::Stmt","","","","mylang_ast::expr","","","","","","","","","","","","","","","mylang_ast::expr::Expr","","","","","","mylang_ast::stmt","","","","","","","","","","","","","","mylang_ast::stmt::Stmt","","",""],"d":["","式を表す抽象構文木","","","","文を表す抽象構文木","","","","","","","","","","","","","","","式を表す抽象構文木","","","","","","","","","","","","","","","","","","","","","","文を表す抽象構文木","","","","","","","","","","","","","","",""],"i":[1,0,1,2,2,0,1,0,0,3,4,5,3,4,5,6,7,6,7,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,3,4,5,3,4,5,2,2,0,2,2,2,2,2,2,2,2,2,2,2,6,7,6,7],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[],["result",4]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["",0]],["range",3]],[[["",0]],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[],["result",4]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["",0]],["range",3]],[[["",0]],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null],"p":[[4,"Expr"],[4,"Stmt"],[13,"I32Lit"],[13,"StrLit"],[13,"Add"],[13,"PrintI32"],[13,"PrintStr"]]},\
"mylang_ast_interp":{"doc":"抽象構文木インタプリタ","t":[4,6,13,11,11,0,5,5,11,11,11,11,11,11,11,11,12,12,12,12,4,13,13,3,4,13,13,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12],"n":["AstInterpError","AstInterpResult","TypeMismatch","borrow","borrow_mut","entity","eval_expr","execute","fmt","fmt","from","into","to_string","try_from","try_into","type_id","actual","expected","0","0","Entity","I32","I32","I32Entity","RuntimeTypeInfo","Str","Str","StrEntity","add","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","get_type","into","into","into","into","new","new","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","0","0"],"q":["mylang_ast_interp","","","","","","","","","","","","","","","","mylang_ast_interp::AstInterpError","","mylang_ast_interp::entity","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","mylang_ast_interp::entity::Entity",""],"d":["","","","","","…","","抽象構文木を解釈実行する","","","","","","","","","","","","","バイトコード実行時に扱う値","","","符号付き32ビット整数値","実行時型情報","","","文字列データ","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,1,1,0,0,0,1,1,1,1,1,1,1,1,2,2,3,4,0,5,6,0,0,5,6,0,3,3,4,5,6,3,4,5,6,3,3,4,4,5,5,6,3,4,5,6,6,3,4,5,6,3,4,3,4,5,3,4,5,6,3,4,5,6,3,4,5,6,7,8],"f":[null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],null,[[["expr",4]],["astinterpresult",6,[["entity",4]]]],[[],["astinterpresult",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,[[["",0],["",0]]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0],["formatter",3]],["fmtresult",6]],[[["",0],["formatter",3]],["fmtresult",6]],[[["",0],["formatter",3]],["fmtresult",6]],[[["",0],["formatter",3]],["fmtresult",6]],[[["",0],["formatter",3]],["fmtresult",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[["",0]],["runtimetypeinfo",4]],[[]],[[]],[[]],[[]],[[["i32",0]]],[[["string",3]]],[[["",0]],["string",3]],[[["",0]],["string",3]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null],"p":[[4,"AstInterpError"],[13,"TypeMismatch"],[3,"I32Entity"],[3,"StrEntity"],[4,"RuntimeTypeInfo"],[4,"Entity"],[13,"I32"],[13,"Str"]]},\
"mylang_bytecode":{"doc":"バイトコードの定義","t":[13,13,13,4,13,13,13,13,11,11,11,11,11,11,11,11,11,11,12,12,12],"n":["Call","I32Add","I32Const","Inst","PrintI32","PrintStr","Return","StrConst","borrow","borrow_mut","deserialize","fmt","from","into","serialize","try_from","try_into","type_id","0","0","0"],"q":["mylang_bytecode","","","","","","","","","","","","","","","","","","mylang_bytecode::Inst","",""],"d":["PC + 1 をスタックに push …","","","命令","","","スタックトから pop した番地にジャンプする","","","","","","","","","","","","","",""],"i":[1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,3,4],"f":[null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[],["result",4]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["",0]],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null],"p":[[4,"Inst"],[13,"I32Const"],[13,"StrConst"],[13,"Call"]]},\
"mylang_bytecode_compiler":{"doc":"抽象構文木→バイトコード変換器","t":[5,5,5],"n":["ast_to_bytecode","expr_to_bytecode","stmt_to_bytecode"],"q":["mylang_bytecode_compiler","",""],"d":["","",""],"i":[0,0,0],"f":[[[],["vec",3,[["inst",4]]]],[[["expr",4]],["vec",3,[["inst",4]]]],[[["stmt",4]],["vec",3,[["inst",4]]]]],"p":[]},\
"mylang_cli_ext":{"doc":"","t":[13,7,4,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Binary","FILE_FORMAT_POSSIBLE_VALUES","FileFormat","Json","borrow","borrow_mut","clone","clone_into","from","from_str","into","to_owned","to_possible_value","try_from","try_into","type_id","value_of","value_variants"],"q":["mylang_cli_ext","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","",""],"i":[1,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["fileformat",4]],[[["",0],["",0]]],[[]],[[["str",0]],["result",4]],[[]],[[["",0]]],[[["",0]],["option",4,[["possiblevalue",3]]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["argmatches",3],["str",0]],["result",4,[["error",3]]]],[[]]],"p":[[4,"FileFormat"]]},\
"mylang_lexer":{"doc":"字句解析器","t":[13,13,13,3,4,8,13,11,11,11,11,11,12,5,10,11,0,0,12,0,11,11,11,0,12,12,12,12,12,12,12,13,13,13,4,6,13,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,13,13,4,13,13,11,11,11,11,11,11,0,11,0,0,11,11,11,11,12,12,12,3,12,11,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,3,12,11,11,11,11,12,12,11,11,11,11,12,11,11,11,11,11,11,3,12,11,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,0,0,0,0,5,13,4,13,11,11,11,5,11,11,11,11,12,12,5,13,13,13,4,11,11,11,11,5,11,11,11,12,12,12,12,13,13,4,11,11,11,11,5,11,11,11,12,12,3,8,11,11,12,11,11,11,11,12,11,11,11,10],"n":["ForbiddenChar","InvalidEscapeSequence","InvalidKeyword","Lex","LexErr","LexExt","MissingClosingQuoteForStr","borrow","borrow_mut","from","into","into_iter","iter","lex","lex","next","result","state","state","transition","try_from","try_into","type_id","with_pos","0","0","0","0","1","1","1","ForbiddenChar","InvalidEscapeSequence","InvalidKeyword","LexErr","LexResult","MissingClosingQuoteForStr","borrow","borrow_mut","fmt","fmt","from","into","locate","to_string","try_from","try_into","type_id","0","0","0","0","1","1","1","I32","Initial","State","Str","Symbol","borrow","borrow_mut","clone","clone_into","fmt","from","i32","into","str","symbol","to_owned","try_from","try_into","type_id","0","0","0","I32State","acc","append_digit_char","borrow","borrow_mut","clone","clone_into","end","fmt","from","into","new","start","to_owned","tokenize","try_from","try_into","type_id","StrState","acc","borrow","borrow_mut","clone","clone_into","end","escape","fmt","from","into","new","start","to_owned","tokenize","try_append_char","try_from","try_into","type_id","SymbolState","acc","append_char","borrow","borrow_mut","clone","clone_into","end","fmt","from","into","new","start","to_owned","tokenize","try_from","try_into","type_id","i32","initial","str","symbol","transition","Continued","I32LexResult","Interrupted","borrow","borrow_mut","from","i32_lex","into","try_from","try_into","type_id","0","0","initial_lex","Completed","Continued","Err","StrLexResult","borrow","borrow_mut","from","into","str_lex","try_from","try_into","type_id","0","0","0","1","Continued","Interrupted","SymbolLexResult","borrow","borrow_mut","from","into","symbol_lex","try_from","try_into","type_id","0","0","WithPos","WithPosExt","borrow","borrow_mut","chars","from","into","into_iter","next","pos","try_from","try_into","type_id","with_pos"],"q":["mylang_lexer","","","","","","","","","","","","","","","","","","","","","","","","mylang_lexer::LexErr","","","","","","","mylang_lexer::result","","","","","","","","","","","","","","","","","mylang_lexer::result::LexErr","","","","","","","mylang_lexer::state","","","","","","","","","","","","","","","","","","","mylang_lexer::state::State","","","mylang_lexer::state::i32","","","","","","","","","","","","","","","","","","mylang_lexer::state::str","","","","","","","","","","","","","","","","","","","mylang_lexer::state::symbol","","","","","","","","","","","","","","","","","","mylang_lexer::transition","","","","","mylang_lexer::transition::i32","","","","","","","","","","","mylang_lexer::transition::i32::I32LexResult","","mylang_lexer::transition::initial","mylang_lexer::transition::str","","","","","","","","","","","","mylang_lexer::transition::str::StrLexResult","","","","mylang_lexer::transition::symbol","","","","","","","","","","","mylang_lexer::transition::symbol::SymbolLexResult","","mylang_lexer::with_pos","","","","","","","","","","","","",""],"d":["","","","…","字句解析中に発生するエラー","…","","","","","","","","字句解析を実行する","…","…","字句解析の結果","字句解析器の状態","","字句解析器内部の状態遷移の実装","","","","…","","","","","","","","","","","字句解析中に発生するエラー","…","","","","","","","","","","","","","","","","","","","","","初期状態","字句解析器内部の状態","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","…","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,1,1,0,0,0,1,2,2,2,2,2,2,0,3,2,0,0,2,0,2,2,2,0,4,5,6,7,4,5,6,1,1,1,0,0,1,1,1,1,1,1,1,1,1,1,1,1,4,5,6,7,4,5,6,8,8,0,8,8,8,8,8,8,8,8,0,8,0,0,8,8,8,8,9,10,11,0,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,0,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,0,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,0,0,0,0,0,15,0,15,15,15,15,0,15,15,15,15,16,17,0,18,18,18,0,18,18,18,18,0,18,18,18,19,20,21,21,22,22,0,22,22,22,22,0,22,22,22,23,24,0,0,25,25,25,25,25,25,25,25,25,25,25,26],"f":[null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[]],[[]],[[]],null,[[]],[[],["lex",3]],[[["",0]],["option",4]],null,null,null,null,[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["",0]],["range",3]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["state",4]],[[["",0],["",0]]],[[["",0],["formatter",3]],["result",6]],[[]],null,[[]],null,null,[[["",0]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null,null,[[["",0],["pos",3],["char",0]]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["i32state",3]],[[["",0],["",0]]],null,[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["pos",3],["string",3]]],null,[[["",0]]],[[["",0]],["token",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["strstate",3]],[[["",0],["",0]]],null,null,[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["pos",3]]],null,[[["",0]]],[[["",0]],["token",4]],[[["",0],["pos",3],["char",0]],["result",4,[["lexerr",4]]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,[[["",0],["pos",3],["char",0]]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["symbolstate",3]],[[["",0],["",0]]],null,[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["pos",3],["string",3]]],null,[[["",0]]],[[["",0]],["token",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null,[[["state",4]]],null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[]],[[["i32state",3]],["i32lexresult",4]],[[]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,[[]],null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[]],[[]],[[["strstate",3]],["strlexresult",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[]],[[]],[[["symbolstate",3]],["symbollexresult",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],null,[[]],[[]],[[]],[[["",0]],["option",4]],null,[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[],["withpos",3]]],"p":[[4,"LexErr"],[3,"Lex"],[8,"LexExt"],[13,"ForbiddenChar"],[13,"InvalidEscapeSequence"],[13,"InvalidKeyword"],[13,"MissingClosingQuoteForStr"],[4,"State"],[13,"I32"],[13,"Str"],[13,"Symbol"],[3,"I32State"],[3,"StrState"],[3,"SymbolState"],[4,"I32LexResult"],[13,"Continued"],[13,"Interrupted"],[4,"StrLexResult"],[13,"Continued"],[13,"Completed"],[13,"Err"],[4,"SymbolLexResult"],[13,"Continued"],[13,"Interrupted"],[3,"WithPos"],[8,"WithPosExt"]]},\
"mylang_lsp_server":{"doc":"mylang を記述するための、LSP …","t":[5,0,0,0,0,4,13,13,13,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,3,11,11,11,11,11,12,11,11,11,11,3,0,11,11,0,12,11,11,0,11,11,0,12,11,11,11,5,5,5,5,5,5,5,5,5,5,5,5,3,11,11,11,11,11,11,11,11],"n":["launch_lsp_server","message","receiver","responder","sender","LspMessage","Notification","Request","Response","borrow","borrow_mut","deserialize","fmt","from","into","raw_message","serialize","try_from","try_into","type_id","id","id","method","method","params","params","result","Receiver","borrow","borrow_mut","from","into","new","responder","started","try_from","try_into","type_id","Responder","analyzer","borrow","borrow_mut","diagnostic","diagnostics_supported","from","handle","handler","into","new","range","sender","try_from","try_into","type_id","analyze_src","lex_all","lex_err_to_diagnostic","parse_err_to_diagnostic","handle","handle_did_change_notification","handle_did_close_notification","handle_did_open_notification","handle_initialize_request","text_document_uri","token_types","locatable_to_lsp_range","Sender","borrow","borrow_mut","from","handle","into","try_from","try_into","type_id"],"q":["mylang_lsp_server","","","","","mylang_lsp_server::message","","","","","","","","","","","","","","","mylang_lsp_server::message::LspMessage","","","","","","","mylang_lsp_server::receiver","","","","","","","","","","","mylang_lsp_server::responder","","","","","","","","","","","","","","","","mylang_lsp_server::responder::analyzer","","mylang_lsp_server::responder::diagnostic","","mylang_lsp_server::responder::handler","","","","","","","mylang_lsp_server::responder::range","mylang_lsp_server::sender","","","","","","","",""],"d":["…","","","","","","","","","","","","","","","","","","","","","","","","","","","…","","","","","","","","","","","…","","","","","","","","","","","","","","","","","","","","","","","","","","","","…","","","","","","","",""],"i":[0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,3,2,4,2,4,3,0,5,5,5,5,5,5,5,5,5,5,0,0,6,6,0,6,6,6,0,6,6,0,6,6,6,6,0,0,0,0,0,0,0,0,0,0,0,0,0,7,7,7,7,7,7,7,7],"f":[[[]],null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[],["result",4]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["",0]],["string",3]],[[["",0]],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[]],[[]],[[["addr",3,[["responder",3]]]]],null,[[["",0]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,[[["",0]],["",0]],[[["",0]],["",0]],null,null,[[]],[[["",0],["lspmessage",4]]],null,[[]],[[["addr",3,[["sender",3]]]]],null,null,[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["addr",3,[["sender",3]]],["jsonvalue",4],["str",0]]],[[["str",0]]],[[["lexerr",4]],["jsonvalue",4]],[[["parseerr",4]],["jsonvalue",4]],[[["addr",3,[["sender",3]]],["arc",3,[["atomicbool",3]]],["lspmessage",4]]],[[["addr",3,[["sender",3]]],["jsonvalue",4]]],[[["addr",3,[["sender",3]]],["jsonvalue",4]]],[[["addr",3,[["sender",3]]],["jsonvalue",4]]],[[["addr",3,[["sender",3]]],["arc",3,[["atomicbool",3]]],["usize",0],["jsonvalue",4]]],[[["jsonvalue",4]],["option",4,[["jsonvalue",4]]]],[[["jsonvalue",4]],["option",4,[["hashmap",3,[["string",3],["usize",0]]]]]],[[["",0]],["jsonvalue",4]],null,[[["",0]],["",0]],[[["",0]],["",0]],[[]],[[["",0],["lspmessage",4],["context",3]]],[[]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]]],"p":[[4,"LspMessage"],[13,"Request"],[13,"Response"],[13,"Notification"],[3,"Receiver"],[3,"Responder"],[3,"Sender"]]},\
"mylang_parser":{"doc":"構文解析器","t":[13,4,6,13,0,5,0,12,12,0,5,0,0,0,5,5,5,5,13,4,6,13,11,11,11,11,11,11,11,11,11,11,11,12,12],"n":["KeywordExpected","ParseErr","ParseResult","TermExpected","parse","parse","result","0","0","expr","parse","program","stmt","term","expr","program","stmt","term","KeywordExpected","ParseErr","ParseResult","TermExpected","borrow","borrow_mut","fmt","fmt","from","into","locate","to_string","try_from","try_into","type_id","0","0"],"q":["mylang_parser","","","","","","","mylang_parser::ParseErr","","mylang_parser::parse","","","","","mylang_parser::parse::expr","mylang_parser::parse::program","mylang_parser::parse::stmt","mylang_parser::parse::term","mylang_parser::result","","","","","","","","","","","","","","","mylang_parser::result::ParseErr",""],"d":["","構文解析中に発生するエラー","","","","","","","","","","","","","","","","","","構文解析中に発生するエラー","","","","","","","","","","","","","","",""],"i":[1,0,0,1,0,0,0,2,3,0,0,0,0,0,0,0,0,0,1,0,0,1,1,1,1,1,1,1,1,1,1,1,1,2,3],"f":[null,null,null,null,null,[[],["vec",3,[["parseresult",6,[["stmt",4]]]]]],null,null,null,null,[[],["vec",3,[["parseresult",6,[["stmt",4]]]]]],null,null,null,[[["putback",3],["pos",3]],["parseresult",6]],[[["putback",3]],["vec",3,[["parseresult",6,[["stmt",4]]]]]],[[["putback",3],["pos",3]],["parseresult",6,[["stmt",4]]]],[[["putback",3],["pos",3]],["parseresult",6]],null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["",0]],["range",3]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null],"p":[[4,"ParseErr"],[13,"TermExpected"],[13,"KeywordExpected"]]},\
"mylang_token":{"doc":"…","t":[13,13,13,13,4,13,8,13,3,13,13,3,13,4,11,11,11,11,12,11,11,12,11,11,11,11,11,11,11,12,0,10,11,0,14,0,14,11,11,12,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,8,10,3,11,11,12,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,3,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11],"n":["AddOp","I32","Ident","Keyword","KeywordKind","Let","Locatable","Newline","Pos","PrintI32","PrintStr","Range","Str","Token","borrow","borrow","borrow_mut","borrow_mut","character","deserialize","deserialize","end","fmt","fmt","from","from","from_str","into","into","line","locatable","locate","locate","pos","pos","range","range","serialize","serialize","start","try_from","try_from","try_into","try_into","type_id","type_id","0","0","0","0","0","0","1","1","1","1","Locatable","locate","Pos","borrow","borrow_mut","character","clone","clone_into","default","deserialize","eq","fmt","fmt","from","into","line","ne","new","next_char","next_line","serialize","to_owned","to_string","try_from","try_into","type_id","Range","borrow","borrow_mut","clone","clone_into","concat","default","deserialize","end","end","end_ref","eq","fmt","fmt","from","from","into","ne","new","serialize","start","start_ref","to_owned","to_string","try_from","try_into","type_id"],"q":["mylang_token","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","mylang_token::Token","","","","","","","","","","mylang_token::locatable","","mylang_token::pos","","","","","","","","","","","","","","","","","","","","","","","","mylang_token::range","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","…","","ソースコード上の位置","","","ソースコード中の範囲","","","","","","","列番号","","","終端","","","","","","","","行番号","","対応する範囲を返す","","","<code>Pos</code> を簡単に生成するためのマクロ","","<code>Range</code> を簡単に生成するためのマクロ","","","始端","","","","","","","","","","","","","","","","","…","対応する範囲を返す","ソースコード上の位置","","","列番号","","","","","","","","","","行番号","","新しい位置を生成して返す","次の文字位置に移動する …","次の行の先頭に移動する","","","","","","","ソースコード中の範囲","","","","","終端を別の範囲の終端に設定する","","","所有権をムーブして終端を返す","終端","終端の参照を返す","","","","始端と終端が同じ位置の範囲に変換する","","","","新しい範囲を生成して返す","","始端","始端の参照を返す","","","","",""],"i":[1,1,1,1,0,2,0,1,0,2,2,0,1,0,2,1,2,1,3,2,1,4,2,1,2,1,2,2,1,3,0,5,1,0,0,0,0,2,1,4,2,1,2,1,2,1,6,7,8,9,10,11,6,8,9,10,0,5,0,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,0,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],null,[[],["result",4]],[[],["result",4]],null,[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["str",0]],["result",4]],[[]],[[]],null,null,[[["",0]],["range",3]],[[["",0]],["range",3]],null,null,null,null,[[["",0]],["result",4]],[[["",0]],["result",4]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["range",3]],null,[[["",0]],["",0]],[[["",0]],["",0]],null,[[["",0]],["pos",3]],[[["",0],["",0]]],[[],["pos",3]],[[],["result",4]],[[["",0],["pos",3]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],null,[[["",0],["pos",3]],["bool",0]],[[["u32",0],["u32",0]]],[[["",0]]],[[["",0]]],[[["",0]],["result",4]],[[["",0]]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["range",3]],[[["",0],["",0]]],[[["",0],["range",3]]],[[],["range",3]],[[],["result",4]],[[],["pos",3]],null,[[["",0]],["pos",3]],[[["",0],["range",3]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["pos",3]]],[[]],[[]],[[["",0],["range",3]],["bool",0]],[[["pos",3],["pos",3]]],[[["",0]],["result",4]],null,[[["",0]],["pos",3]],[[["",0]]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]]],"p":[[4,"Token"],[4,"KeywordKind"],[3,"Pos"],[3,"Range"],[8,"Locatable"],[13,"I32"],[13,"AddOp"],[13,"Str"],[13,"Keyword"],[13,"Ident"],[13,"Newline"]]},\
"mylang_vm":{"doc":"バイトコードインタプリタ","t":[13,13,4,6,11,11,5,0,5,11,11,11,5,5,11,5,5,5,5,11,11,11,11,12,12,12,12,12,13,13,4,13,13,3,4,13,13,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12],"n":["StackUnderflow","TypeMismatch","VMError","VMResult","borrow","borrow_mut","call","entity","execute","fmt","fmt","from","i32_add","i32_const","into","print_i32","print_str","ret","str_const","to_string","try_from","try_into","type_id","0","actual","expected","0","0","Addr","Addr","Entity","I32","I32","I32Entity","RuntimeTypeInfo","Str","Str","StrEntity","add","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","get_type","into","into","into","into","new","new","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","0","0","0"],"q":["mylang_vm","","","","","","","","","","","","","","","","","","","","","","","mylang_vm::VMError","","","mylang_vm::entity","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","mylang_vm::entity::Entity","",""],"d":["…","…","…","バイトコードの実行結果","","","","仮想マシンで扱う「実体」の定義","バイトコードを解釈実行する","","","","I32Add 命令を実行する","I32Const 命令を実行する","","PrintI32 命令を実行する","PrintStr 命令を実行する","","StrConst 命令を実行する","","","","","","","","","","","","バイトコード実行時に扱う値","","","符号付き32ビット整数値","実行時型情報","","","文字列データ","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,1,0,0,1,1,0,0,0,1,1,1,0,0,1,0,0,0,0,1,1,1,1,2,3,3,4,5,6,7,0,6,7,0,0,6,7,0,4,4,5,6,7,4,5,6,7,4,4,5,5,6,6,7,4,5,6,7,7,4,5,6,7,4,5,4,5,6,4,5,6,7,4,5,6,7,4,5,6,7,8,9,10],"f":[null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["vec",3],["usize",0],["usize",0]],["result",4,[["vmerror",4]]]],null,[[],["result",4,[["vmerror",4]]]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[["vec",3]],["result",4,[["vmerror",4]]]],[[["vec",3],["i32",0]]],[[]],[[["vec",3]],["result",4,[["vmerror",4]]]],[[["vec",3]],["result",4,[["vmerror",4]]]],[[["vec",3],["usize",0]],["result",4,[["vmerror",4]]]],[[["vec",3],["str",0]]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0],["",0]]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0],["formatter",3]],["fmtresult",6]],[[["",0],["formatter",3]],["fmtresult",6]],[[["",0],["formatter",3]],["fmtresult",6]],[[["",0],["formatter",3]],["fmtresult",6]],[[["",0],["formatter",3]],["fmtresult",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[["",0]],["runtimetypeinfo",4]],[[]],[[]],[[]],[[]],[[["i32",0]]],[[["string",3]]],[[["",0]],["string",3]],[[["",0]],["string",3]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,null],"p":[[4,"VMError"],[13,"StackUnderflow"],[13,"TypeMismatch"],[3,"I32Entity"],[3,"StrEntity"],[4,"RuntimeTypeInfo"],[4,"Entity"],[13,"Addr"],[13,"I32"],[13,"Str"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};