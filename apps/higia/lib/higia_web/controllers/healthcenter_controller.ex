defmodule HigiaWeb.HealthcenterController do
  use HigiaWeb, :controller

  def index(conn, _params) do
    render(conn, :index)
  end

  def show(conn, %{"center_name" => center_name}) do
    render(conn, :show, center_name: center_name)
  end
end
