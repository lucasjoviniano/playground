defmodule LevenshteinDistance do
  @moduledoc """
  Documentation for `LevenshteinDistance`.
  """

  @doc """
  Hello world.
  """
  @spec distance(String.t(), String.t()) :: integer
  def distance("", snd), do: String.length(snd)

  @spec distance(String.t(), String.t()) :: integer
  def distance(fst, ""), do: String.length(fst)

  @spec distance(String.t(), String.t()) :: integer
  def distance(fst, snd) do
    [fst_one, rest_one] = pattern_str(fst)
    [fst_two, rest_two] = pattern_str(snd)

    if fst_one == fst_two do
      distance(rest_one, rest_two)
    else
      result = [
        distance(fst, rest_two),
        distance(rest_one, snd),
        distance(rest_one, rest_two)
      ]

      1 + Enum.min(result)
    end
  end

  defp pattern_str(str) when is_binary(str) do
    <<head, tail::binary>> = str

    [<<head>>, tail]
  end

end
