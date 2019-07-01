const path = require('path')
const tape = require('tape')
const tapSpec = require('tap-spec')

tape.createStream()
  .pipe(tapSpec())
  .pipe(process.stdout)

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

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

diorama.registerScenario("Can create and play games", async (s, t, {alice, bob}) => {
  console.log("  Alice creates a game with Bob")
  const create_game_result_1 = await alice.callSync('main', 'create_game', {
    opponent: bob.agentId,
    timestamp: 0
  })
  console.log(create_game_result_1)
  t.equal(create_game_result_1.Ok.length, 46)
  const game_address_1 = create_game_result_1.Ok
  var game_state = await alice.callSync('main', 'get_state',{
    game_address: game_address_1
  })
  console.log(game_state)

  console.log("  Alice tries to make a move before her turn")
  const bad_move_1_result = await alice.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: { Place: { x: 0, y: 0 } },
      timestamp: 1
    }
  })
  console.log(bad_move_1_result)
  t.equal(bad_move_1_result.Ok, undefined)
  game_state = await alice.callSync('main', 'get_state',{
    game_address: game_address_1
  })
  console.log(game_state)

  console.log("  Bob makes the first move")
  const move_1_result = await bob.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: { Place: { x: 0, y: 0 } },
      timestamp: 2
    }
  })
  console.log(move_1_result)
  t.equal(move_1_result.Err, undefined)
  game_state = await bob.callSync('main', 'get_state',{
    game_address: game_address_1
  })
  console.log(game_state)

  console.log("  Alice tries to make an invalid move")
  const bad_move_2_result = await alice.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: { Place: { x: 0, y: 0 } },
      timestamp: 3
    }
  })
  console.log(bad_move_2_result)
  t.equal(bad_move_2_result.Ok, undefined)
  game_state = await alice.callSync('main', 'get_state',{
    game_address: game_address_1
  })
  console.log(game_state)

  console.log("  Alice makes a valid move")
  const move_2_result = await alice.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: { Place: { x: 1, y: 1 } },
      timestamp: 4
    }
  })
  console.log(move_2_result)
  t.equal(move_2_result.Err, undefined)
  game_state = await alice.callSync('main', 'get_state',{
    game_address: game_address_1
  })
  console.log(game_state)

  console.log("  Bob makes an invalid move out of bounds")
  const bad_move_3_result = await bob.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: { Place: { x: 3, y: 0 } },
      timestamp: 5
    }
  })
  console.log(bad_move_3_result)
  t.equal(bad_move_3_result.Ok, undefined)
  game_state = await bob.callSync('main', 'get_state',{
    game_address: game_address_1
  })
  console.log(game_state)

  console.log("  Bob makes a valid move")
  const move_3_result = await bob.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: { Place: { x: 2, y: 0 } },
      timestamp: 6
    }
  })
  console.log(move_3_result)
  t.equal(move_3_result.Err, undefined)
  game_state = await bob.callSync('main', 'get_state',{
    game_address: game_address_1
  })
  console.log(game_state)

  console.log("  Alice makes a stupid but valid move")
  const move_4_result = await alice.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: { Place: { x: 1, y: 2 } },
      timestamp: 7
    }
  })
  console.log(move_4_result)
  t.equal(move_4_result.Err, undefined)
  game_state = await alice.callSync('main', 'get_state',{
    game_address: game_address_1
  })
  console.log(game_state)

  console.log("  Bob makes a valid move and wins")
  const move_5_result = await bob.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: { Place: { x: 1, y: 0 } },
      timestamp: 8
    }
  })
  console.log(move_5_result)
  t.equal(move_5_result.Err, undefined)
  game_state = await bob.callSync('main', 'get_state',{
    game_address: game_address_1
  })
  console.log(game_state)

  console.log("  Alice tries to move again but the game is over")
  const bad_move_6_result = await alice.callSync('main', 'make_move', {
    new_move: {
      game: game_address_1,
      move_type: { Place: { x: 0, y: 1 } },
      timestamp: 9
    }
  })
  console.log(bad_move_6_result)
  t.equal(bad_move_6_result.Ok, undefined)
  game_state = await alice.callSync('main', 'get_state',{
    game_address: game_address_1
  })
  console.log(game_state)

  console.log("  Bob creates a game with Alice")
  const create_game_result_2 = await bob.callSync('main', 'create_game', {
    opponent: alice.agentId,
    timestamp: 10
  })
  console.log(create_game_result_2)
  t.equal(create_game_result_2.Ok.length, 46)
  const game_address_2 = create_game_result_2.Ok
  game_state = await bob.callSync('main', 'get_state',{
    game_address: game_address_2
  })
  console.log(game_state)

  console.log("  Alice resigns immediately")
  const resign_result = await alice.callSync('main', 'make_move', {
    new_move: {
      game: game_address_2,
      move_type: 'Resign',
      timestamp: 11
    }
  })
  console.log(resign_result)
  t.equal(resign_result.Err, undefined)
  game_state = await alice.callSync('main', 'get_state',{
    game_address: game_address_2
  })
  console.log(game_state)
})

diorama.run()
