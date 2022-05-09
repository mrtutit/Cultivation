import React from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import './App.css'
import './custom.css'

// Config
import { getConfig } from '../utils/configuration'

// Major Components
import Topbar from './components/TopBar'
import BigButton from './components/common/BigButton'
import Checkbox from './components/common/Checkbox'

async function playGame() {
  const config = await getConfig()

  if (!config.game_path) return

  // Launch the program
  await invoke('run_program', { path: config.game_path })
}

function App() {
  return (
    <div className="App">
      <Topbar />
      <div id="playButton">
        <div id="serverControls">
          <Checkbox label="Connect via Grasscutter" />
        </div>
        <BigButton text="PLAY DA GAME :D" onClick={playGame} id="officialPlay" />
      </div>
    </div>
  )
}

export default App
