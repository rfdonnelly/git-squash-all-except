# frozen_string_literal: true

directory 'tush' do
  sh 'git clone https://github.com/adolfopa/tush'
  sh 'git -C tush checkout -q f3b594c2974e594fc0e190f58d57b9c2e1851001'
end

TUSH_PATH = File.join(__dir__, %w[tush bin])
SHELL_PATH = File.join(__dir__, 'shell')
RUST_PATH = File.join(__dir__, %w[rust target debug])
TEST_PATH = File.join(__dir__, %w[tests])
ENV['PATH'] = [
  TUSH_PATH,
  RUST_PATH,
  SHELL_PATH,
  TEST_PATH,
  ENV['PATH'],
].join(':')
ROOT_PATH = File.dirname(File.expand_path(__FILE__))
TESTS = FileList['tests/*_test.adoc']
TESTS.each do |test|
  task test => 'tush' do
    sh "ROOT_PATH=#{ROOT_PATH} runtest #{test}"
  end
end

multitask test: TESTS

multitask default: :test
