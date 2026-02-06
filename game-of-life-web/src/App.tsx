import React, { useState, useEffect, useRef } from 'react';
import init, { Life, WasmPoint } from '../wasm/wasm.js';

import './App.css'

interface GameOfLifeProps {
  width?: number;
  height?: number;
  cellSize?: number;
}

function App({ width = 20, height = 20, cellSize = 20 }: GameOfLifeProps) {
  const [life, setLife] = useState<Life | null>(null);
  const [cells, setCells] = useState<Uint32Array>(new Uint32Array(0));
  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [speed, setSpeed] = useState<number>(100);
  const [generation, setGeneration] = useState<number>(0);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animationRef = useRef<number | null>(null);
  const lifeRef = useRef<Life | null>(null);

  useEffect(() => {
    async function loadWasm() {
      try {
        await init();

        const initialCells: WasmPoint[] = [
          new WasmPoint(1, 0),
          new WasmPoint(2, 1),
          new WasmPoint(0, 2),
          new WasmPoint(1, 2),
          new WasmPoint(2, 2),
        ];

        const game: Life = new Life(initialCells, width, height);
        setLife(game);
        lifeRef.current = game;

        const initialCellsArray: Uint32Array = game.tick();
        setCells(initialCellsArray);
        setGeneration(1);

        console.log('WASM initialized successfully');
      } catch (error) {
        console.error('Failed to initialize WASM:', error);
      }
    }

    loadWasm();
  }, [width, height]);

  useEffect(() => {
    if (isRunning && lifeRef.current) {
      animationRef.current = window.setInterval(() => {
        const newCells: Uint32Array = lifeRef.current!.tick();
        setCells(newCells);
        setGeneration(prev => prev + 1);
      }, speed);
    } else {
      if (animationRef.current) {
        clearInterval(animationRef.current);
        animationRef.current = null;
      }
    }

    return () => {
      if (animationRef.current) {
        clearInterval(animationRef.current);
      }
    };
  }, [isRunning, speed]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.fillStyle = '#1a1a2e';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.fillStyle = '#4ecca3';
    for (let i = 0; i < cells.length; i += 2) {
      const x = cells[i];
      const y = cells[i + 1];
      ctx.fillRect(x * cellSize, y * cellSize, cellSize - 1, cellSize - 1);
    }
  }, [cells, cellSize]);

  const toggleRunning = (): void => {
    setIsRunning(!isRunning);
  };

  const reset = (): void => {
    if (lifeRef.current) {
      const initialCells: WasmPoint[] = [
        new WasmPoint(1, 0),
        new WasmPoint(2, 1),
        new WasmPoint(0, 2),
        new WasmPoint(1, 2),
        new WasmPoint(2, 2),
      ];
      const newLife: Life = new Life(initialCells, width, height);
      setLife(newLife);
      lifeRef.current = newLife;

      const initialCellsArray: Uint32Array = newLife.tick();
      setCells(initialCellsArray);
      setGeneration(1);
      setIsRunning(false);
    }
  };

  const clear = (): void => {
    if (lifeRef.current) {
      const emptyCells: WasmPoint[] = [];
      const newLife: Life = new Life(emptyCells, width, height);
      setLife(newLife);
      lifeRef.current = newLife;

      const initialCellsArray: Uint32Array = newLife.tick();
      setCells(initialCellsArray);
      setGeneration(1);
      setIsRunning(false);
    }
  };

  const handleCanvasClick = (e: React.MouseEvent<HTMLCanvasElement>): void => {
    const canvas = canvasRef.current;
    if (!canvas || !lifeRef.current) return;

    const rect = canvas.getBoundingClientRect();
    const x = Math.floor((e.clientX - rect.left) / cellSize);
    const y = Math.floor((e.clientY - rect.top) / cellSize);

    if (x >= width || y >= height) return;

    const currentCells = Array.from(cells);
    let found = false;

    for (let i = 0; i < currentCells.length; i += 2) {
      if (currentCells[i] === x && currentCells[i + 1] === y) {
        currentCells.splice(i, 2);
        found = true;
        break;
      }
    }

    if (!found) {
      currentCells.push(x, y);
    }

    const wasmPoints: WasmPoint[] = [];
    for (let i = 0; i < currentCells.length; i += 2) {
      wasmPoints.push(new WasmPoint(currentCells[i], currentCells[i + 1]));
    }

    const newLife: Life = new Life(wasmPoints, width, height);
    setLife(newLife);
    lifeRef.current = newLife;
    setCells(new Uint32Array(currentCells));
    setGeneration(1);
    setIsRunning(false);
  };

  const handleSpeedChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
    setSpeed(Number(e.target.value));
  };

  return (
    <div style={{
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center',
      justifyContent: 'center',
      minHeight: '100vh',
      backgroundColor: '#16213e',
      color: '#e94560',
      fontFamily: 'Arial, sans-serif',
      padding: '20px'
    }}>
      <h1 style={{ marginBottom: '20px', color: '#4ecca3' }}>Conway's Game of Life</h1>

      <div style={{ marginBottom: '20px', color: '#fff' }}>
        <span>Generation: {generation}</span>
        <span style={{ marginLeft: '20px' }}>Live Cells: {cells.length / 2}</span>
      </div>

      <canvas
        ref={canvasRef}
        width={width * cellSize}
        height={height * cellSize}
        onClick={handleCanvasClick}
        style={{
          border: '2px solid #4ecca3',
          borderRadius: '8px',
          cursor: 'pointer',
          marginBottom: '20px'
        }}
      />

      <div style={{ marginBottom: '20px' }}>
        <button
          onClick={toggleRunning}
          disabled={!life}
          style={{
            margin: '5px',
            padding: '10px 20px',
            fontSize: '16px',
            backgroundColor: isRunning ? '#e94560' : '#4ecca3',
            color: '#fff',
            border: 'none',
            borderRadius: '5px',
            cursor: life ? 'pointer' : 'not-allowed',
            opacity: life ? 1 : 0.5
          }}
        >
          {isRunning ? 'Pause' : 'Start'}
        </button>
        <button
          onClick={reset}
          disabled={!life}
          style={{
            margin: '5px',
            padding: '10px 20px',
            fontSize: '16px',
            backgroundColor: '#0f3460',
            color: '#fff',
            border: 'none',
            borderRadius: '5px',
            cursor: life ? 'pointer' : 'not-allowed',
            opacity: life ? 1 : 0.5
          }}
        >
          Reset
        </button>
        <button
          onClick={clear}
          disabled={!life}
          style={{
            margin: '5px',
            padding: '10px 20px',
            fontSize: '16px',
            backgroundColor: '#0f3460',
            color: '#fff',
            border: 'none',
            borderRadius: '5px',
            cursor: life ? 'pointer' : 'not-allowed',
            opacity: life ? 1 : 0.5
          }}
        >
          Clear
        </button>
      </div>

      <div style={{ marginBottom: '20px', color: '#fff' }}>
        <label htmlFor="speed" style={{ marginRight: '10px' }}>
          Speed: {speed}ms
        </label>
        <input
          id="speed"
          type="range"
          min="10"
          max="500"
          value={speed}
          onChange={handleSpeedChange}
          style={{ width: '200px' }}
        />
      </div>

      <div style={{ color: '#aaa', fontSize: '14px', textAlign: 'center' }}>
        <p>Click on the grid to toggle cells</p>
        <p>Rules: Live cells with 2-3 neighbors survive, dead cells with exactly 3 neighbors become alive</p>
      </div>

      {!life && (
        <div style={{
          position: 'fixed',
          top: '50%',
          left: '50%',
          transform: 'translate(-50%, -50%)',
          backgroundColor: 'rgba(0, 0, 0, 0.8)',
          color: '#fff',
          padding: '20px',
          borderRadius: '10px',
          zIndex: 1000
        }}>
          Loading WASM...
        </div>
      )}
    </div>
  );
}

export default App
