package fluence.network

import cats.syntax.show._
import fluence.kad._
import fluence.network.client.{ KademliaClient, NetworkClient }
import fluence.network.proto.kademlia.KademliaGrpc
import fluence.network.server.{ KademliaServerImpl, KademliaService, NetworkServer }
import monix.execution.Scheduler.Implicits.global
import org.slf4j.LoggerFactory

import scala.concurrent.duration._
import scala.io.StdIn
import scala.util.{ Failure, Success }

object MainApp extends App {
  // get port from config
  // get kademlia constants from config

  // get key seed from config

  // launch app

  // take (external) connection info
  // serialize ip and port
  // base64 it
  // println: it's a seed

  // invite to join this node with a seed

  // operations:
  //   join other seed
  //   get seed
  //   find a node by a (raw) key
  //   send a message by a (raw) key
  // once message is received, println (from, message) with a color

  val log = LoggerFactory.getLogger(getClass)

  val serverBuilder = NetworkServer.builder

  // For demo purposes
  val rawKey = StdIn.readLine(Console.CYAN + "Who are you?\n> " + Console.RESET)

  val key = Key.sha1(rawKey.getBytes)

  val header = serverBuilder.header(key)

  // We have only Kademlia service client
  val client = NetworkClient.builder
    .add(KademliaClient.register(header))
    .build

  // This is a server service, not a client
  val kad = new KademliaService(
    key,
    serverBuilder.contact,
    KademliaClient(client),
    KademliaConf.read()
  )

  // Add server (with kademlia inside), build
  val server = serverBuilder
    .add(KademliaGrpc.bindService(new KademliaServerImpl(kad), global))
    .build

  sys.addShutdownHook {
    log.warn("*** shutting down gRPC server since JVM is shutting down")
    server.shutdown(10.seconds)
    log.warn("*** server shut down")
  }

  server.start().attempt.runAsync.foreach {
    case Left(err) ⇒
      log.error("Can't launch server", err)

    case Right(_) ⇒
      log.info("Server launched")

      server.contact.foreach{ contact ⇒
        println("Your contact is: " + contact.show)
        println("You may share this seed for others to join you: " + Console.MAGENTA + contact.b64seed + Console.RESET)
      }
  }

  def cmd(s: String): Unit = println(Console.BLUE + s + Console.RESET)

  while (true) {

    cmd("join(j) / lookup(l)")

    StdIn.readLine() match {
      case "j" | "join" ⇒
        cmd("join seed?")
        val p = StdIn.readLine()
        Contact.readB64seed(p).attempt.value match {
          case Left(err) ⇒
            log.error("Can't read the seed", err)

          case Right(c) ⇒
            kad.join(Seq(c), 16).runAsync.onComplete {
              case Success(_) ⇒ cmd("ok")
              case Failure(e) ⇒
                log.error("Can't join", e)
            }
        }

      case "l" | "lookup" ⇒
        cmd("lookup myself")
        kad.handleRPC
          .lookup(Key.XorDistanceMonoid.empty, 10)
          .map(_.map(_.show).mkString("\n"))
          .map(println)
          .runAsync.onComplete {
            case Success(_) ⇒ println("ok")
            case Failure(e) ⇒
              log.error("Can't lookup", e)
          }

      case "s" | "seed" ⇒
        server.contact.map(_.b64seed).runAsync.foreach(cmd)

      case "q" | "quit" | "x" | "exit" ⇒
        cmd("exit")
        System.exit(0)

      case _ ⇒
    }
  }
}