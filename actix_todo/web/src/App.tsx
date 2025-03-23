import { Routes, Route, Outlet, Link, NavLink } from "react-router-dom";
import { cx } from "class-variance-authority";

import Todos from "@/pages/todos";
import { Button } from "./components/ui/button";

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<Layout />}>
        <Route index element={<Home />} />
        <Route path="about" element={<About />} />
        <Route path="todos" element={<Todos />} />
        <Route path="*" element={<NoMatch />} />
      </Route>
    </Routes>
  );
}

function Layout() {
  return (
    <div>
      <nav>
        <ul className="flex gap-x-4">
          <li>
            <NavLink to="/">
              {(props) => {
                return (
                  <Button
                    className={cx("cursor-pointer", {
                      underline: props.isActive,
                    })}
                    variant="link"
                  >
                    Home
                  </Button>
                );
              }}
            </NavLink>
          </li>
          <li>
            <NavLink to="/todos">
              {(props) => {
                return (
                  <Button
                    className={cx("cursor-pointer", {
                      underline: props.isActive,
                    })}
                    variant="link"
                  >
                    Todos
                  </Button>
                );
              }}
            </NavLink>
          </li>
          <li>
            <NavLink to="/about">
              {(props) => {
                return (
                  <Button
                    className={cx("cursor-pointer", {
                      underline: props.isActive,
                    })}
                    variant="link"
                  >
                    About
                  </Button>
                );
              }}
            </NavLink>
          </li>
        </ul>
      </nav>
      <hr />
      <Outlet />
    </div>
  );
}

function Home() {
  return (
    <div>
      <h2>
        Welcome to the Todos app! Click on the links above to navigate to
        different pages.
      </h2>
    </div>
  );
}

function About() {
  return (
    <div>
      <h2>
        This is a Todos app built with React, Vite, and Tailwind CSS. It
        demonstrates how to use the React Router.
      </h2>
    </div>
  );
}

function NoMatch() {
  return (
    <div>
      <h2>Nothing to see here!</h2>
      <p>
        <Link to="/">
          <Button variant="link" className="cursor-pointer">
            Go to the home page
          </Button>
        </Link>
      </p>
    </div>
  );
}
