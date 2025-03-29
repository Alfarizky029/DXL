import type { Express } from "express";
import { createServer, type Server } from "http";
import { storage } from "./storage";

export async function registerRoutes(app: Express): Promise<Server> {
  // API routes for our application
  const apiRouter = app.route('/api');
  
  // Fetch educational resources
  app.get('/api/resources', (req, res) => {
    res.json({
      success: true,
      resources: [
        {
          id: 1,
          title: 'Web3 Fundamentals',
          type: 'course',
          description: 'Learn the core concepts of Web3 and understand how it differs from the traditional internet.',
          modules: 8,
          duration: '4 hours'
        },
        {
          id: 2,
          title: 'Blockchain Security',
          type: 'course',
          description: 'Explore security considerations for blockchain projects and learn how to protect your assets.',
          modules: 10,
          duration: '6 hours'
        },
        {
          id: 3,
          title: 'DeFi Masterclass',
          type: 'course',
          description: 'Dive into Decentralized Finance (DeFi) and understand how it\'s reshaping traditional financial systems.',
          modules: 12,
          duration: '8 hours'
        }
      ]
    });
  });
  
  // Get learning paths
  app.get('/api/learning-paths', (req, res) => {
    res.json({
      success: true,
      paths: [
        {
          id: 1,
          title: 'Developer Path',
          description: 'Become a Web3 developer with this structured learning path.',
          steps: [
            { title: 'Web3 Fundamentals', duration: '4 hours' },
            { title: 'Smart Contract Development', duration: '8 hours' },
            { title: 'dApp Building', duration: '12 hours' },
            { title: 'Advanced Security', duration: '6 hours' }
          ]
        },
        {
          id: 2,
          title: 'Business Path',
          description: 'Learn how Web3 can transform business models and operations.',
          steps: [
            { title: 'Web3 Business Models', duration: '3 hours' },
            { title: 'Tokenomics & Incentives', duration: '5 hours' },
            { title: 'DAOs & Governance', duration: '6 hours' },
            { title: 'Web3 Strategy Implementation', duration: '8 hours' }
          ]
        }
      ]
    });
  });
  
  // Get AI assistant responses
  app.post('/api/assistant/query', (req, res) => {
    const query = req.body.query;
    
    // In a real application, this would connect to an AI service
    // For now, we'll use a simple mapping of common questions
    const responses = {
      'blockchain': 'Blockchain is a distributed ledger technology that allows data to be stored globally on thousands of servers. Think of it as a chain of blocks, where each block contains data that is verified and encrypted.',
      'web3': 'Web3 refers to the next generation of the internet, built on decentralized blockchain technology, giving users more control over their data and digital assets.',
      'smart contracts': 'Smart contracts are self-executing contracts with the terms directly written into code. They automatically enforce and execute agreements when predetermined conditions are met.',
      'nft': 'NFTs (Non-Fungible Tokens) are unique digital assets that represent ownership of a specific item or piece of content on the blockchain. Unlike cryptocurrencies such as Bitcoin, each NFT has a distinct value and cannot be exchanged on a like-for-like basis.',
      'defi': 'Decentralized Finance (DeFi) refers to financial services that operate on blockchain networks without traditional intermediaries like banks or brokerages. DeFi applications aim to create an open, permissionless financial system.',
      'default': 'I\'m your Web3 learning assistant. Let me help you explore more about that topic in our educational modules!'
    };
    
    let response = responses.default;
    
    // Simple keyword matching
    if (query) {
      const lowerQuery = query.toLowerCase();
      for (const [key, value] of Object.entries(responses)) {
        if (lowerQuery.includes(key)) {
          response = value;
          break;
        }
      }
    }
    
    res.json({
      success: true,
      response: response
    });
  });

  const httpServer = createServer(app);

  return httpServer;
}
