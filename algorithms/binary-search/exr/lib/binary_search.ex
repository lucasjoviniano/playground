defmodule BinarySearch do
  @moduledoc """
  Documentation for `BinarySearch`.
  """

  @spec search(tuple, integer, integer, integer) :: :not_found
  defp search(list, target, left, right) when left > right, do: :not_found

  @spec search(tuple, integer, integer, integer) :: {:ok, integer} | :not_found
  defp search(list, target, left, right) do
    mid = div(left + right, 2)

    cond do
      target == elem(list, mid) -> {:ok, mid}
      target < elem(list, mid) -> search(list, target, left, mid - 1)
      target > elem(list, mid) -> search(list, target, mid + 1, right)
    end
  end

  @spec search(tuple, integer) :: :not_found
  def search({}, target), do: :not_found

  @spec search(tuple, integer) :: {:ok, integer} | :not_found
  def search(list, target) do
    right = tuple_size(list) - 1
    elems = list |> Tuple.to_list |> Enum.sort |> List.to_tuple

    search(elems, target, 0, right)
  end
end
