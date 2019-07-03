const path = require('path')
const tape = require('tape')
const tapSpec = require('tap-spec')

tape.createStream()
  .pipe(tapSpec())
  .pipe(process.stdout)

const {
  Diorama,
  tapeExecutor,
  backwardCompatibilityMiddleware
} = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/tic-tac-toe.dna.json")
const dna = Diorama.dna(dnaPath, 'tic-tac-toe')

const diorama = new Diorama({
  instances: {
    alice: dna,
    bob: dna,
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

diorama.registerScenario("Can create and play games", async (s, t, {
  alice,
  bob
}) => {
  console.log("Alice creates a game with Bob".underline)
  const create_game_result_1 = await alice.callSync('main', 'create_game', {
    opponent: bob.agentId,
    timestamp: 0
  })
  console.log(create_game_result_1)
  t.equal(create_game_result_1.Ok.length, 46)
  const game_address_1 = create_game_result_1.Ok
  var game_state = await alice.callSync('main', 'render_state', {
    game_address: game_address_1
  })
  console.log(game_state.Ok)

  console.log("Alice tries to make a move before her turn".underline)
  const bad_move_1_result = await alice.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: {
        Place: {
          x: 4,
          y: 5
        }
      },
      timestamp: 1
    }
  })
  console.log(bad_move_1_result)
  t.equal(bad_move_1_result.Ok, undefined)
  game_state = await alice.callSync('main', 'render_state', {
    game_address: game_address_1
  })
  console.log(game_state.Ok)

  console.log("Bob makes the first move".underline)
  const move_1_result = await bob.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: {
        Place: {
          x: 4,
          y: 2
        }
      },
      timestamp: 2
    }
  })
  console.log(move_1_result)
  t.equal(move_1_result.Err, undefined)
  game_state = await bob.callSync('main', 'render_state', {
    game_address: game_address_1
  })
  console.log(game_state.Ok)

  console.log("Alice tries to make an invalid move".underline)
  const bad_move_2_result = await alice.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: {
        Place: {
          x: 5,
          y: 3
        }
      },
      timestamp: 3
    }
  })
  console.log(bad_move_2_result)
  t.equal(bad_move_2_result.Ok, undefined)
  game_state = await alice.callSync('main', 'render_state', {
    game_address: game_address_1
  })
  console.log(game_state.Ok)

  console.log("Alice makes a valid move".underline)
  const move_2_result = await alice.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: {
        Place: {
          x: 5,
          y: 2
        }
      },
      timestamp: 4
    }
  })
  console.log(move_2_result)
  t.equal(move_2_result.Err, undefined)
  game_state = await alice.callSync('main', 'render_state', {
    game_address: game_address_1
  })
  console.log(game_state.Ok)

  console.log("Bob makes an invalid move out of bounds".underline)
  const bad_move_3_result = await bob.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: {
        Place: {
          x: 3,
          y: 8
        }
      },
      timestamp: 5
    }
  })
  console.log(bad_move_3_result)
  t.equal(bad_move_3_result.Ok, undefined)
  game_state = await bob.callSync('main', 'render_state', {
    game_address: game_address_1
  })
  console.log(game_state.Ok)

  console.log("Bob makes a valid move".underline)
  const move_3_result = await bob.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: {
        Place: {
          x: 6,
          y: 2
        }
      },
      timestamp: 6
    }
  })
  console.log(move_3_result)
  t.equal(move_3_result.Err, undefined)
  game_state = await bob.callSync('main', 'render_state', {
    game_address: game_address_1
  })
  console.log(game_state.Ok)

  console.log("Alice passes".underline)
  const pass_1_result = await alice.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: 'Pass',
      timestamp: 7
    }
  })
  console.log(pass_1_result)
  t.equal(pass_1_result.Err, undefined)
  game_state = await alice.callSync('main', 'render_state', {
    game_address: game_address_1
  })
  console.log(game_state.Ok)

  console.log("Bob passes".underline)
  const pass_2_result = await bob.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: 'Pass',
      timestamp: 8
    }
  })
  console.log(resign_result)
  t.equal(pass_2_result.Err, undefined)
  game_state = await bob.callSync('main', 'render_state', {
    game_address: game_address_1
  })
  console.log(game_state.Ok)

  console.log("Alice tries to move again but the game is over".underline)
  const bad_move_6_result = await alice.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: {
        Place: {
          x: 6,
          y: 1
        }
      },
      timestamp: 9
    }
  })
  console.log(bad_move_6_result)
  t.equal(bad_move_6_result.Ok, undefined)
  game_state = await alice.callSync('main', 'render_state', {
    game_address: game_address_1
  })
  console.log(game_state.Ok)
})

diorama.run()
