(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/gracalc.js":
/*!*************************!*\
  !*** ../pkg/gracalc.js ***!
  \*************************/
/*! exports provided: __wbg_set_wasm, greet, AdjList, Edge, Vertex, __wbg_log_2ba3a5c05dd47201, __wbg_alert_cbfbbd7838bc4997, __wbindgen_string_new, __wbg_edge_new, __wbg_vertex_new, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _gracalc_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./gracalc_bg.wasm */ \"../pkg/gracalc_bg.wasm\");\n/* harmony import */ var _gracalc_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./gracalc_bg.js */ \"../pkg/gracalc_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_set_wasm\", function() { return _gracalc_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_set_wasm\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"greet\", function() { return _gracalc_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"greet\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"AdjList\", function() { return _gracalc_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"AdjList\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"Edge\", function() { return _gracalc_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"Edge\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"Vertex\", function() { return _gracalc_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"Vertex\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_2ba3a5c05dd47201\", function() { return _gracalc_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_log_2ba3a5c05dd47201\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_alert_cbfbbd7838bc4997\", function() { return _gracalc_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_alert_cbfbbd7838bc4997\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_new\", function() { return _gracalc_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_string_new\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_edge_new\", function() { return _gracalc_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_edge_new\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_vertex_new\", function() { return _gracalc_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_vertex_new\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _gracalc_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n\n\nObject(_gracalc_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_set_wasm\"])(_gracalc_bg_wasm__WEBPACK_IMPORTED_MODULE_0__);\n\n\n\n//# sourceURL=webpack:///../pkg/gracalc.js?");

/***/ }),

/***/ "../pkg/gracalc_bg.js":
/*!****************************!*\
  !*** ../pkg/gracalc_bg.js ***!
  \****************************/
/*! exports provided: __wbg_set_wasm, greet, AdjList, Edge, Vertex, __wbg_log_2ba3a5c05dd47201, __wbg_alert_cbfbbd7838bc4997, __wbindgen_string_new, __wbg_edge_new, __wbg_vertex_new, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_set_wasm\", function() { return __wbg_set_wasm; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"greet\", function() { return greet; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"AdjList\", function() { return AdjList; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Edge\", function() { return Edge; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Vertex\", function() { return Vertex; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_2ba3a5c05dd47201\", function() { return __wbg_log_2ba3a5c05dd47201; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_alert_cbfbbd7838bc4997\", function() { return __wbg_alert_cbfbbd7838bc4997; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_new\", function() { return __wbindgen_string_new; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_edge_new\", function() { return __wbg_edge_new; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_vertex_new\", function() { return __wbg_vertex_new; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\nlet wasm;\nfunction __wbg_set_wasm(val) {\n    wasm = val;\n}\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8Memory0 = null;\n\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nconst heap = new Array(128).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nlet heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n/**\n*/\nfunction greet() {\n    wasm.greet();\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length, 1) >>> 0;\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len, 1) >>> 0;\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n        ptr = realloc(ptr, len, offset, 1) >>> 0;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachedInt32Memory0 = null;\n\nfunction getInt32Memory0() {\n    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {\n        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);\n    }\n    return cachedInt32Memory0;\n}\n\nfunction getObject(idx) { return heap[idx]; }\n\nfunction dropObject(idx) {\n    if (idx < 132) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nfunction _assertClass(instance, klass) {\n    if (!(instance instanceof klass)) {\n        throw new Error(`expected instance of ${klass.name}`);\n    }\n    return instance.ptr;\n}\n\nlet cachedUint32Memory0 = null;\n\nfunction getUint32Memory0() {\n    if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {\n        cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);\n    }\n    return cachedUint32Memory0;\n}\n\nfunction getArrayJsValueFromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    const mem = getUint32Memory0();\n    const slice = mem.subarray(ptr / 4, ptr / 4 + len);\n    const result = [];\n    for (let i = 0; i < slice.length; i++) {\n        result.push(takeObject(slice[i]));\n    }\n    return result;\n}\n\nconst AdjListFinalization = (typeof FinalizationRegistry === 'undefined')\n    ? { register: () => {}, unregister: () => {} }\n    : new FinalizationRegistry(ptr => wasm.__wbg_adjlist_free(ptr >>> 0));\n/**\n*/\nclass AdjList {\n\n    static __wrap(ptr) {\n        ptr = ptr >>> 0;\n        const obj = Object.create(AdjList.prototype);\n        obj.__wbg_ptr = ptr;\n        AdjListFinalization.register(obj, obj.__wbg_ptr, obj);\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.__wbg_ptr;\n        this.__wbg_ptr = 0;\n        AdjListFinalization.unregister(this);\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        wasm.__wbg_adjlist_free(ptr);\n    }\n    /**\n    * @returns {AdjList}\n    */\n    static new() {\n        const ret = wasm.adjlist_new();\n        return AdjList.__wrap(ret);\n    }\n    /**\n    * @param {string} prog\n    * @returns {AdjList}\n    */\n    static try_parse(prog) {\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            const ptr0 = passStringToWasm0(prog, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n            const len0 = WASM_VECTOR_LEN;\n            wasm.adjlist_try_parse(retptr, ptr0, len0);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            var r2 = getInt32Memory0()[retptr / 4 + 2];\n            if (r2) {\n                throw takeObject(r1);\n            }\n            return AdjList.__wrap(r0);\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n        }\n    }\n    /**\n    * @returns {string}\n    */\n    getAsString() {\n        let deferred1_0;\n        let deferred1_1;\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            wasm.adjlist_getAsString(retptr, this.__wbg_ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            deferred1_0 = r0;\n            deferred1_1 = r1;\n            return getStringFromWasm0(r0, r1);\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);\n        }\n    }\n    /**\n    * @param {Vertex} vertex\n    */\n    insert_vertex(vertex) {\n        _assertClass(vertex, Vertex);\n        var ptr0 = vertex.__destroy_into_raw();\n        wasm.adjlist_add_vertex(this.__wbg_ptr, ptr0);\n    }\n    /**\n    * @param {Vertex} vertex\n    * @returns {boolean}\n    */\n    contains_vertex(vertex) {\n        _assertClass(vertex, Vertex);\n        const ret = wasm.adjlist_contains_vertex(this.__wbg_ptr, vertex.__wbg_ptr);\n        return ret !== 0;\n    }\n    /**\n    * @param {Vertex} vertex\n    * @param {Vertex} vertex2\n    * @returns {boolean}\n    */\n    is_adjacent(vertex, vertex2) {\n        _assertClass(vertex, Vertex);\n        _assertClass(vertex2, Vertex);\n        const ret = wasm.adjlist_is_adjacent(this.__wbg_ptr, vertex.__wbg_ptr, vertex2.__wbg_ptr);\n        return ret !== 0;\n    }\n    /**\n    * @param {Vertex} vertex\n    * @returns {(Vertex)[] | undefined}\n    */\n    get_predecessors(vertex) {\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            _assertClass(vertex, Vertex);\n            wasm.adjlist_get_predecessors(retptr, this.__wbg_ptr, vertex.__wbg_ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            let v1;\n            if (r0 !== 0) {\n                v1 = getArrayJsValueFromWasm0(r0, r1).slice();\n                wasm.__wbindgen_free(r0, r1 * 4, 4);\n            }\n            return v1;\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n        }\n    }\n    /**\n    * @param {Vertex} vertex\n    * @returns {(Vertex)[] | undefined}\n    */\n    get_children(vertex) {\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            _assertClass(vertex, Vertex);\n            wasm.adjlist_get_children(retptr, this.__wbg_ptr, vertex.__wbg_ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            let v1;\n            if (r0 !== 0) {\n                v1 = getArrayJsValueFromWasm0(r0, r1).slice();\n                wasm.__wbindgen_free(r0, r1 * 4, 4);\n            }\n            return v1;\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n        }\n    }\n    /**\n    * @param {Vertex} vertex\n    */\n    add_vertex(vertex) {\n        _assertClass(vertex, Vertex);\n        var ptr0 = vertex.__destroy_into_raw();\n        wasm.adjlist_add_vertex(this.__wbg_ptr, ptr0);\n    }\n    /**\n    * @param {Vertex} vertex\n    */\n    remove_vertex(vertex) {\n        _assertClass(vertex, Vertex);\n        var ptr0 = vertex.__destroy_into_raw();\n        wasm.adjlist_remove_vertex(this.__wbg_ptr, ptr0);\n    }\n    /**\n    * @param {Vertex} v1\n    * @param {Vertex} v2\n    */\n    add_edge(v1, v2) {\n        _assertClass(v1, Vertex);\n        _assertClass(v2, Vertex);\n        wasm.adjlist_add_edge(this.__wbg_ptr, v1.__wbg_ptr, v2.__wbg_ptr);\n    }\n    /**\n    * @param {Vertex} v1\n    * @param {Vertex} v2\n    */\n    remove_edge_between(v1, v2) {\n        _assertClass(v1, Vertex);\n        _assertClass(v2, Vertex);\n        wasm.adjlist_remove_edge_between(this.__wbg_ptr, v1.__wbg_ptr, v2.__wbg_ptr);\n    }\n    /**\n    * @returns {(Vertex)[]}\n    */\n    get_vertices() {\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            wasm.adjlist_get_vertices(retptr, this.__wbg_ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();\n            wasm.__wbindgen_free(r0, r1 * 4, 4);\n            return v1;\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n        }\n    }\n    /**\n    * @returns {(Edge)[]}\n    */\n    get_edges() {\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            wasm.adjlist_get_edges(retptr, this.__wbg_ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();\n            wasm.__wbindgen_free(r0, r1 * 4, 4);\n            return v1;\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n        }\n    }\n    /**\n    * @returns {number}\n    */\n    get_vertex_len() {\n        const ret = wasm.adjlist_get_vertex_len(this.__wbg_ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @param {Vertex} vertex\n    * @returns {(Vertex)[]}\n    */\n    dfs(vertex) {\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            _assertClass(vertex, Vertex);\n            wasm.adjlist_dfs(retptr, this.__wbg_ptr, vertex.__wbg_ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();\n            wasm.__wbindgen_free(r0, r1 * 4, 4);\n            return v1;\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n        }\n    }\n    /**\n    * @param {Vertex} vertex\n    * @returns {(Vertex)[]}\n    */\n    bfs(vertex) {\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            _assertClass(vertex, Vertex);\n            wasm.adjlist_bfs(retptr, this.__wbg_ptr, vertex.__wbg_ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();\n            wasm.__wbindgen_free(r0, r1 * 4, 4);\n            return v1;\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n        }\n    }\n    /**\n    * @returns {string}\n    */\n    get_svg() {\n        let deferred1_0;\n        let deferred1_1;\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            wasm.adjlist_get_svg(retptr, this.__wbg_ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            deferred1_0 = r0;\n            deferred1_1 = r1;\n            return getStringFromWasm0(r0, r1);\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);\n        }\n    }\n}\n\nconst EdgeFinalization = (typeof FinalizationRegistry === 'undefined')\n    ? { register: () => {}, unregister: () => {} }\n    : new FinalizationRegistry(ptr => wasm.__wbg_edge_free(ptr >>> 0));\n/**\n*/\nclass Edge {\n\n    static __wrap(ptr) {\n        ptr = ptr >>> 0;\n        const obj = Object.create(Edge.prototype);\n        obj.__wbg_ptr = ptr;\n        EdgeFinalization.register(obj, obj.__wbg_ptr, obj);\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.__wbg_ptr;\n        this.__wbg_ptr = 0;\n        EdgeFinalization.unregister(this);\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        wasm.__wbg_edge_free(ptr);\n    }\n}\n\nconst VertexFinalization = (typeof FinalizationRegistry === 'undefined')\n    ? { register: () => {}, unregister: () => {} }\n    : new FinalizationRegistry(ptr => wasm.__wbg_vertex_free(ptr >>> 0));\n/**\n*/\nclass Vertex {\n\n    static __wrap(ptr) {\n        ptr = ptr >>> 0;\n        const obj = Object.create(Vertex.prototype);\n        obj.__wbg_ptr = ptr;\n        VertexFinalization.register(obj, obj.__wbg_ptr, obj);\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.__wbg_ptr;\n        this.__wbg_ptr = 0;\n        VertexFinalization.unregister(this);\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        wasm.__wbg_vertex_free(ptr);\n    }\n    /**\n    * @param {string} val\n    * @returns {Vertex}\n    */\n    static new(val) {\n        const ptr0 = passStringToWasm0(val, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n        const len0 = WASM_VECTOR_LEN;\n        const ret = wasm.vertex_new(ptr0, len0);\n        return Vertex.__wrap(ret);\n    }\n}\n\nfunction __wbg_log_2ba3a5c05dd47201(arg0, arg1) {\n    console.log(getStringFromWasm0(arg0, arg1));\n};\n\nfunction __wbg_alert_cbfbbd7838bc4997(arg0, arg1) {\n    alert(getStringFromWasm0(arg0, arg1));\n};\n\nfunction __wbindgen_string_new(arg0, arg1) {\n    const ret = getStringFromWasm0(arg0, arg1);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_edge_new(arg0) {\n    const ret = Edge.__wrap(arg0);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_vertex_new(arg0) {\n    const ret = Vertex.__wrap(arg0);\n    return addHeapObject(ret);\n};\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/gracalc_bg.js?");

/***/ }),

/***/ "../pkg/gracalc_bg.wasm":
/*!******************************!*\
  !*** ../pkg/gracalc_bg.wasm ***!
  \******************************/
/*! exports provided: memory, __wbg_vertex_free, __wbg_edge_free, vertex_new, __wbg_adjlist_free, adjlist_new, adjlist_try_parse, adjlist_getAsString, adjlist_contains_vertex, adjlist_is_adjacent, adjlist_get_predecessors, adjlist_get_children, adjlist_add_vertex, adjlist_remove_vertex, adjlist_add_edge, adjlist_remove_edge_between, adjlist_get_vertices, adjlist_get_edges, adjlist_get_vertex_len, adjlist_dfs, adjlist_bfs, adjlist_get_svg, adjlist_insert_vertex, greet, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_add_to_stack_pointer, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./gracalc_bg.js */ \"../pkg/gracalc_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/gracalc_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var gracalc__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! gracalc */ \"../pkg/gracalc.js\");\n\n\nfunction drawGraph(ret){\n    let elem = document.getElementById(\"img\");\n    console.log(elem);\n    elem.innerHTML = ret.get_svg();\n    \n    Array.from(document.getElementById(\"nodes\").getElementsByTagName(\"g\")).forEach(n => {\n        const [name,vertex1,vertex2] = n.id.split(\"--\");\n        if (name===\"node\"){\n            n.addEventListener(\"click\", () => {\n                console.log(\"vertex \" + vertex1 +\" got clicked\");\n                vertexClicked(ret,vertex1);\n            });}\n        else if( name ==\"edge\"){\n            n.addEventListener(\"click\", () => {\n                console.log(\"edge \" + vertex1 +\" \"+ vertex2 +\" got clicked\");\n                edgeClicked(ret,vertex1,vertex2);});\n        }else{\n            console.log(\"something fishy going on\");\n        }\n    });\n}\n\nconst input_text = document.getElementById(\"text_area\");\n\nlet vertexClicked=(graph,vertex)=>{\n    console.log(self);\n    if (self.count)\n        self.count +=1;\n    else{ self.count = 1;}\n    if (self.last_click && self.count && self.count == 2){\n        graph.add_edge(gracalc__WEBPACK_IMPORTED_MODULE_0__[\"Vertex\"].new(self.last_click),gracalc__WEBPACK_IMPORTED_MODULE_0__[\"Vertex\"].new(vertex));\n        input_text.value = graph.getAsString();\n        drawGraph(graph);\n        self.count=0;\n    }\n    self.last_click =  vertex;\n};\nlet edgeClicked=(graph,vertex1,vertex2)=>{\n    console.log(\"edge \" + vertex1 +  \"--\" +vertex2 + \"clicked\");\n    graph.remove_edge_between(gracalc__WEBPACK_IMPORTED_MODULE_0__[\"Vertex\"].new(vertex1),gracalc__WEBPACK_IMPORTED_MODULE_0__[\"Vertex\"].new(vertex2));\n    //const input_text = document.getElementById(\"text_area\");\n    const gr = graph.getAsString();\n    input_text.value = gr;\n    console.log(gr);\n    drawGraph(graph);\n};\n\nconst errElem = document.createElement(\"textarea\");\nerrElem.setAttribute(\"id\", \"ErrDiv\");\nerrElem.setAttribute(\"cols\", \"80\");\nerrElem.setAttribute(\"rows\", \"30\");\nerrElem.setAttribute(\"readonly\", \"true\");\n\n\ninput_text.addEventListener(\"input\", (event) => {\n    try{\n        let ret = gracalc__WEBPACK_IMPORTED_MODULE_0__[\"AdjList\"].try_parse(event.target.value);\n        drawGraph(ret);\n        if (document.body.contains(errElem)){\n            let errDiv = document.getElementById(\"ErrDiv\");\n            errDiv.remove();\n        }\n    }catch (errorMessage){\n        if (document.body.contains(errElem)){\n            const   errElem = document.getElementById(\"ErrDiv\");\n            errElem.textContent = errorMessage ;\n        }else{\n            errElem.textContent = errorMessage ;\n            input_text.parentNode.appendChild(errElem);\n        }\n    }\n});\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);