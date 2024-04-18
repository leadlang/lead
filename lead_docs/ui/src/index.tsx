/* @refresh reload */
import { render } from 'solid-js/web'

import './index.css'
import App from './app/index'

const root = document.getElementById('root')

render(() => <App />, root!)
