defmodule Higia.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      HigiaWeb.Telemetry,
      Higia.Repo,
      {DNSCluster, query: Application.get_env(:higia, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Higia.PubSub},
      # Start a worker by calling: Higia.Worker.start_link(arg)
      # {Higia.Worker, arg},
      # Start to serve requests, typically the last entry
      HigiaWeb.Endpoint
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Higia.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    HigiaWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
