import { users, User, UserDetail, user } from "wasm"
import { Component, createSignal, onMount, Show } from 'solid-js';
import { Button } from "./components/button"

const [items, setItems] = createSignal<User[]>([])
const [item, setItem] = createSignal<UserDetail | null>(null)

const setItems_ = () => {
  users().then((res: User[]) => {
    setItems(res)
    setItem(null)
  })
}

const setItem_ = (login: string) => {
  user(login).then((res: UserDetail) => {
    console.log(res)
    setItem(res)
  })
}

const App: Component = () => {
  onMount(() => setItems_())
  return (
    <>
      <Button onClick={setItems_}>
        reload
      </Button>
      <div class="flex">
        <div>
          {items().map((i) => {
            return <div class="p-4 flex items-center" onClick={() => setItem_(i.login)}>
              <img src={i.avatar_url} class="w-24 h-24 rounded-full" />
              <h1 class="pl-4">{i.login}</h1>
            </div>
          })}
        </div>
        <Show when={!!item()}>
          <div>
            <img src={item()?.avatar_url} class="w-24 h-24 rounded-full" />
            <table>
              <tbody>
                <tr>
                  <th>名前</th>
                  <td>{item()?.name}</td>
                </tr>
                <tr>
                  <th>フォロワー数</th>
                  <td>{item()?.followers}人</td>
                </tr>
                <tr>
                  <th>フォロー数</th>
                  <td>{item()?.following}人</td>
                </tr>
              </tbody>
            </table>
          </div>
        </Show>
      </div >
    </>
  );
};

export default App;
