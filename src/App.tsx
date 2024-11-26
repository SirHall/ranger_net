import type { Component } from "solid-js";

import logo from "./logo.svg";
import styles from "./App.module.css";
import "lume";

const App: Component = () => {

  const peer = new Peer(crypto.randomUUID());

  
  
  return (
    <>
      <lume-scene
        webgl
        physically-correct-lights
        perspective="800"
        fog-mode="linear"
        fog-color="#8338ec"
        fog-near="600"
        fog-far="900"
      >
        <lume-camera-rig align-point="0.5 0.5" distance="800"></lume-camera-rig>

        <lume-point-light
          intensity="1200"
          align-point="0.5 0.5"
          position="300 -300 300"
          color="#ff006e"
        >
          <lume-sphere
            size="20"
            cast-shadow="false"
            receive-shadow="false"
            color="#ff006e"
            has="basic-material"
          ></lume-sphere>
        </lume-point-light>

        <lume-point-light
          intensity="1200"
          align-point="0.5 0.5"
          position="-300 300 -300"
          color="#3a86ff"
        >
          <lume-sphere
            size="20"
            cast-shadow="false"
            receive-shadow="false"
            color="#3a86ff"
            has="basic-material"
          ></lume-sphere>
        </lume-point-light>

        <lume-point-light
          intensity="1200"
          align-point="0.5 0.5"
          position="-300 300 300"
          color="#3a86ff"
        >
          <lume-sphere
            size="20"
            cast-shadow="false"
            receive-shadow="false"
            color="#3a86ff"
            has="basic-material"
          ></lume-sphere>
        </lume-point-light>

        <lume-point-light
          intensity="1200"
          align-point="0.5 0.5"
          position="300 -300 -300"
          color="#ff006e"
        >
          <lume-sphere
            size="20"
            cast-shadow="false"
            receive-shadow="false"
            color="#ff006e"
            has="basic-material"
          ></lume-sphere>
        </lume-point-light>

        <lume-box
          id="box"
          cast-shadow="false"
          receive-shadow="false"
          has="physical-material"
          roughness="0.8"
          align-point="0.5 0.5"
          mount-point="0.5 0.5 0.5"
          size="200 200 200"
          color="white"
          position="0 0 -500"
        ></lume-box>
      </lume-scene>
    </>
    // <div class={styles.App}>
    //   <header class={styles.header}>
    //     <img src={logo} class={styles.logo} alt="logo" />
    //     <p>
    //       Edit <code>src/App.tsx</code> and save to reload.
    //     </p>
    //     <a
    //       class={styles.link}
    //       href="https://github.com/solidjs/solid"
    //       target="_blank"
    //       rel="noopener noreferrer"
    //     >
    //       Learn Solid
    //     </a>
    //   </header>
    // </div>
  );
};

export default App;
