#include <iostream>
#include <memory>
#include <mutex>
#include <thread>
#include <vector>

struct Table {
  std::vector<std::unique_ptr<std::mutex>> forks;
  Table(unsigned int n) {
    for (int i = 0; i < n; i++) {
      forks.push_back(std::make_unique<std::mutex>());
    }
  }
};

struct Philosopher {
  std::string name;
  unsigned int left;
  unsigned int right;

  Philosopher(const std::string &name, unsigned int left, unsigned int right)
      : name(name), left(left), right(right) {}

  void eat(Table &table) {
    std::scoped_lock _left(*table.forks[this->left]);
    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
    std::scoped_lock _right(*table.forks[this->right]);

    std::cout << this->name << " is eating!\n";

    std::this_thread::sleep_for(std::chrono::milliseconds(1000));

    std::cout << this->name << " is done eating!\n";
  }

  std::thread spawn(std::shared_ptr<Table> &table) {
    return std::thread([this, &table] { this->eat(*table.get()); });
  }
};

int main() {
  constexpr unsigned int n_of_philosophers = 5;

  auto table =
      std::shared_ptr<Table>(std::make_shared<Table>(n_of_philosophers));

  static_assert(std::is_same_v<decltype(table), std::shared_ptr<Table>>);

  std::vector<Philosopher> philos{{"Immanuel Kant", 0, 1},
                                  {"Karl Marx", 1, 2},
                                  {"Simone de Beauvoir", 2, 3},
                                  {"René Descartes", 3, 4},
                                  {"Hannah Arendt", 0, 4}};

  std::vector<std::thread> handles;

  for (auto &philo : philos) {
    auto th = philo.spawn(table);
    handles.push_back(std::move(th));
  }

  for (auto &t : handles) {
    t.join();
  }

  return 0;
}
