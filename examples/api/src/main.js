// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import 'uno.css'
import './app.css'
import App from './App.svelte'
import Streaming from './views/Streaming.svelte'
import { mount } from 'svelte'

// Simple router based on URL path
const path = window.location.pathname

let component
let props = {}

if (path === '/streaming') {
  component = Streaming
  props = {
    onMessage: (msg) => console.log('[Streaming]', msg)
  }
} else {
  component = App
}

const app = mount(component, {
  target: document.querySelector('#app'),
  props
})

export default app
