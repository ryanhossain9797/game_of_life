import React, { useState, useEffect, useRef } from 'react';
import init, { Life, WasmPoint } from '../wasm/wasm.js';

import './App.css'

function App() {
  useEffect(() => {
    async function loadWasm() {
      await init();

      const initialCells: WasmPoint[] = [
        new WasmPoint(1, 0),
        new WasmPoint(2, 1),
        new WasmPoint(0, 2),
        new WasmPoint(1, 2),
        new WasmPoint(2, 2),
      ];
    }
    loadWasm();
  }, []);


  return (
    <>
      <div>
        <h1>Hello World</h1>
      </div>
    </>
  )
}

export default App
