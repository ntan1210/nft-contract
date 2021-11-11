import React from 'react'
import { useState, useEffect } from 'react'
import { login, logout } from './utils'
import './global.css'
import Metadata from './Components/Metadata.js'
import { BrowserRouter as Router, Route, Routes, Link, BrowserRouter } from "react-router-dom";
import Market from './Components/Market'
import MyNFT from './Components/MyNFT'
import Mint from './Components/Mint'

import getConfig from './config'
import { async } from 'regenerator-runtime'
const { networkId } = getConfig(process.env.NODE_ENV || 'development')

export default function App() {
    const [status, setStatus] = useState(false);

    useEffect(() => {
        setStatus(window.walletConnection.isSignedIn());
    }, [window.walletConnection.isSignedIn()]);

    if (status) return (
        <div>
            <div className="menu">
                <h2>NFT-MARKET</h2> 
                <div className="tab">
                    <a href="/"><h4>Market</h4></a>
                    <a href="/mynft"><h4>My NFT </h4></a>
                    <a href="/mint"><h4>Mint</h4></a>
                </div>
                <h4 onClick={logout}>
                    <a href="#">Logout</a>
                </h4>
            </div>
            <hr />

            <Router>
                <Routes>
                    <Route path="/" element={<Market />} />
                    <Route path="/mynft" element={<MyNFT />} />
                    <Route path="/mint" element={<Mint />} />
                </Routes>
            </Router>
        </div>

    );
    else return (
        <div>
        <h1>Please log in to use this app</h1>
        <h2 onClick={login}>
            <a href="#">Login</a>
        </h2>
        </div>
    );
}
