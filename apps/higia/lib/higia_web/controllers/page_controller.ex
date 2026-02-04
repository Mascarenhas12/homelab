defmodule HigiaWeb.PageController do
  use HigiaWeb, :controller

  def home(conn, _params) do
    render(conn, :home)
  end
end
