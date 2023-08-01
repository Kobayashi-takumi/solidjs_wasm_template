import { Component, JSX } from "solid-js"

export const Button: Component<JSX.IntrinsicElements["button"]> = (props) => {
  return <button {...props} class="bg-indigo-700 font-semibold text-white py-2 px-4 mx-2 rounded">{props?.children}</button>
}